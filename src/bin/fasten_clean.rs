extern crate getopts;
extern crate fasten;
extern crate multiqueue;
extern crate threadpool;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;

use fasten::fasten_base_options;
//use fasten::logmsg;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = fasten_base_options();

    // invisible option but maybe we can expose it:
    // how many reads to queue at once to the threads.
    let num_reads_per_buffer:usize = 1000;

    // script-specific options
    opts.optopt("","min-length","Minimum length for each read in bp","INT");
    opts.optopt("","min-avg-quality","Minimum average quality for each read","FLOAT");
    opts.optopt("","min-trim-quality","Trim the edges of each read until a nucleotide of at least X quality is found","INT");

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");

    if matches.opt_present("h") {
        println!("{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let mut min_length :usize = 0;
    if matches.opt_present("min-length") {
        min_length = matches.opt_str("min-length")
            .expect("ERROR: could not read the minimum length parameter")
            .parse()
            .expect("ERROR: min-length is not an integer");
    }

    let mut min_avg_qual :f32 = 0.0;
    if matches.opt_present("min-avg-quality") {
        min_avg_qual = matches.opt_str("min-avg-quality")
            .expect("ERROR: could not read the minimum average quality parameter")
            .parse()
            .expect("ERROR: min-avg-qual is not an integer");
    }

    let mut min_trim_qual :u8 = 0;
    if matches.opt_present("min-trim-quality") {
        min_trim_qual = matches.opt_str("min-trim-quality")
            .expect("ERROR: could not read the minimum trim quality parameter")
            .parse()
            .expect("ERROR: min-trim-qual is not an integer");
    }

    let lines_per_read :u8={
        if matches.opt_present("paired-end") {
            8
        }else{
            4
        }
    };
    let num_cpus:usize = {
        if matches.opt_present("numcpus") {
            matches.opt_str("numcpus").expect("ERROR parsing --numcpus")
                .parse()
                .expect("ERROR: numcpus is not an integer")
        } else {
            1
        }
    };

    let (tx, rx):(std::sync::mpsc::Sender<String>,std::sync::mpsc::Receiver<String>) = channel();
    let pool = ThreadPool::new(num_cpus);

    // Read the file and send seqs to threads
    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    
    let mut buffer_iter = my_buffer.lines();
    let num_lines_per_buffer = num_reads_per_buffer * lines_per_read as usize;
    while let Some(line_res) = buffer_iter.next(){
        // get 4 or 8 lines of the fastq into the vector
        let mut lines:Vec<String> = Vec::with_capacity(num_lines_per_buffer);

        let line = line_res.expect("ERROR reading the first line of the next fastq entry");
        lines.push(line);
        for _ in 0..(num_lines_per_buffer-1) {
          if let Some(line_res) = buffer_iter.next() {
            let line =  line_res.expect("ERROR reading the first line of the next fastq entry");
            lines.push(line);
          //} else {
          //  panic!("Expected another line in this fastq entry but got {} out of {}", i+1, lines_per_read);
          }
        }

        // Pass the 4 or 8 lines to a thread
        let tx2 = tx.clone();
        pool.execute(move|| {
          clean_entry(lines, min_length, min_avg_qual, min_trim_qual, lines_per_read, tx2);
        });
    }

    pool.join();
    drop(tx); // signal the end of the transmitting to the channel

    for entry in rx.iter(){
      println!("{}", entry);
    }
}

/// Cleans a SE or PE read
fn clean_entry(lines:Vec<String>, min_length:usize, min_avg_qual:f32, min_trim_qual:u8, lines_per_read:u8, tx:std::sync::mpsc::Sender<String>) {
  let short_blank_string:String = String::with_capacity(100);
  let  long_blank_string:String = String::with_capacity(10000);

  let mut id1   :String = short_blank_string.clone();
  let mut id2   :String = short_blank_string.clone();
  let mut seq1  :String = long_blank_string.clone();
  let mut seq2  :String = long_blank_string.clone();
  let mut qual1 :String = long_blank_string.clone();
  let mut qual2 :String = long_blank_string.clone();

  let mut i = 0;
  for line in lines {

    //let line = wrapped_line.expect("ERROR: could not read line");
    match i % lines_per_read {
        // read ID
        0=>{
            // On the zeroth line, set the first ID...
            id1 = line;
            // ...but then reset all other fields
            id2   = short_blank_string.clone();
            seq1  = long_blank_string.clone();
            seq2  = long_blank_string.clone();
            qual1 = long_blank_string.clone();
            qual2 = long_blank_string.clone();
        }
        4=>{
            id2 = line;
        }

        // Sequence
        1=>{
            seq1 = line;
        }
        5=>{
            seq2 = line;
        }

        // Qual line. If we've gotten here, then we can also trim/filter/print
        // First qual line
        3=>{
            qual1 = line;

        }
        // Second qual line
        7=>{
            qual2 = line;

        }
        2=>{} // + line
        6=>{} // + line
        _=>{
          panic!("Internal error: somehow there are more than 8 lines per entry. The last line was line {} and contents were {}", i, line);
        }


    }

    // moment of truth: see trim the reads and then see
    // if they pass the filter.
    if (lines_per_read==4 && !qual1.is_empty())
     ||(lines_per_read==8 && !qual2.is_empty()) {
        
        // Trim
        let (seq1_trimmed, qual1_trimmed) = 
              trim(&seq1,&qual1,min_trim_qual);

        // If this is single end, go ahead and filter/print
        if lines_per_read==4 {
            if seq1_trimmed.len() >= min_length && avg_quality(&qual1_trimmed) >= min_avg_qual {
                tx.send(format!("{}\n{}\n+\n{}",
                     id1,seq1_trimmed,qual1_trimmed,
                     ))
                   .unwrap();
            }
        } 

        else if lines_per_read==8 {

          // trim second read
          let (seq2_trimmed, qual2_trimmed) = 
                trim(&seq2,&qual2,min_trim_qual);


          // Since we are at the second qual line, this is PE and we can
          // go ahead with filter/print and not check for the PE param.

          if seq2_trimmed.len() >= min_length && seq2_trimmed.len() >= min_length 
              && avg_quality(&qual1_trimmed) >= min_avg_qual 
              && avg_quality(&qual2_trimmed) >= min_avg_qual {

              tx.send(format!("{}\n{}\n+\n{}\n{}\n{}\n+\n{}",
                   id1,seq1_trimmed,qual1_trimmed,
                   id2,seq2_trimmed,qual2_trimmed
                   ))
               .unwrap();
          }
        }
        else {
          panic!("I do not understand {} lines per fastq entry", lines_per_read);
        }

    }
    
    i += 1;
  }
}

/// determine average quality of a qual cigar string
fn avg_quality(qual: &String) -> f32 {
    let mut total :u32 = 0;
    for qual_char in qual.chars() {
        total += qual_char as u8 as u32;
    }
    let avg = (total as f32 / qual.len() as f32) - 33.0;
    return avg;
}

/// Trim the ends of reads with low quality
fn trim(seq: &String, qual: &String, min_qual: u8) -> (String,String) {
    let mut trim5 :usize=0;
    let mut trim3 :usize=qual.len();

    let offset_min_qual = min_qual + 33;
    
    // 5'
    for qual in qual.chars(){
        if (qual as u8) < offset_min_qual {
            trim5+=1;
        } else {
            break;
        }
    }

    // 3'
    for qual in qual.chars().rev() {
        if (qual as u8) < offset_min_qual {
            trim3-=1;
        } else {
            break;
        }
    }

    let new_seq :String;
    let new_qual:String;
    
    if trim5 >= trim3 {
        new_seq = String::new();
        new_qual= String::new();
    } else {
        new_seq  =  seq[trim5..trim3].to_string();
        new_qual = qual[trim5..trim3].to_string();
    }
    return(new_seq,new_qual);
}

