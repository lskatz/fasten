extern crate ross;
extern crate getopts;
extern crate rand;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

use std::env;
use std::f32;

use ross::ross_base_options;
//use ross::logmsg;

// need this constant because the compiler had a problem
// with the syntax 10.0.pow()
const TEN: f32 = 10.0;
const READ_SEPARATOR :&'static str = "~~~~";
const READ_SEPARATOR_LENGTH :usize = 4;

fn main(){
    let args: Vec<String> = env::args().collect();
    let opts = ross_base_options();

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");

    if matches.opt_present("h") {
        println!("Emma: collapse identical reads into single reads, recalculating quality values. If paired end, then each set of reads must be identical. Rachel's daughter Emma was played by twins, essentially collapsing two individuals into one character!");
        println!("{}",opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let lines_per_read={
        if matches.opt_present("paired-end") {
            8
        }else{
            4
        }
    };

    // TODO? keep track of IDs
    // sequence => count
    let mut seq_count:HashMap<String,u32>   =HashMap::new();
    // sequence => quality cigar String
    let mut seq_qual :HashMap<String,String>=HashMap::new();

    let mut current_seq = String::new();
    let mut current_qual= String::new();

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut line_counter =0;
    for line in my_buffer.lines() {
        match line_counter % lines_per_read {
            1 => {
                current_seq.push_str(&line.expect("Could not unwrap seq line"));
            }
            5 => {
                current_seq.push_str(READ_SEPARATOR);
                current_seq.push_str(&line.expect("Could not unwrap seq line"));
            }

            // 3 and 7: the qual lines
            3 => {
                current_qual.push_str(&line.expect("Could not unwrap qual line"));

                // Add an entry if SE
                if lines_per_read == 4 {
                    let count = seq_count.entry(current_seq.clone()).or_insert(0);
                    *count += 1;
                    //println!("{} <= {}",*count, &current_seq);

                    // TODO collect all qual lines and combine them later
                    seq_qual.entry(current_seq.clone()).or_insert(current_qual.clone());

                    current_qual=String::new();
                    current_seq= String::new();
                }
            }
            7 => {
                current_qual.push_str(READ_SEPARATOR);
                current_qual.push_str(&line.expect("Could not unwrap qual line"));

                // Add an entry here since it's PE
                let count = seq_count.entry(current_seq.clone()).or_insert(0);
                *count += 1;
                
                // TODO collect all qual lines and combine them later
                seq_qual.entry(current_seq.clone()).or_insert(current_qual.clone());

                current_qual=String::new();
                current_seq= String::new();
            }
            _=>{}
        }
        line_counter += 1;
    }

    let mut id_counter=0;
    //println!("{:?}", seq_count);
    for (sequence, count) in seq_count {
        let qual = seq_qual.entry(sequence.clone()).or_insert(String::new());
        if qual == "" {
            panic!("ERROR: quality cigar string not found for {}",&sequence);
        }
        id_counter += 1;

        match lines_per_read {
            4=>{
                println!("@read{} collapsed_reads:{}\n{}\n+\n{}",&id_counter,&count,&sequence,&qual);
            }
            8=>{
                // Split the sequences and quals into separate reads.
                let read1_length = sequence.find(READ_SEPARATOR).expect("ERROR finding read separator");
                let sequence1 = &sequence[0..read1_length];
                let quality1  = recalculate_qual(&qual[0..read1_length],count);
                let sequence2 = &sequence[read1_length+READ_SEPARATOR_LENGTH..];
                let quality2  = recalculate_qual(&qual[read1_length+READ_SEPARATOR_LENGTH..],count);

                println!("@read{}/1 collapsed_reads:{}\n{}\n+\n{}",&id_counter,&count,&sequence1,&quality1);
                println!("@read{}/2 collapsed_reads:{}\n{}\n+\n{}",&id_counter,&count,&sequence2,&quality2);
            }
            _=>{
                panic!("INTERNAL ERROR: number of lines per entry is {}, but it should be either 4 or 8",lines_per_read);
            }
        }
    }
}

        /*

    for (entry,count) in entries {
        let mut lines = entry.lines();
        let mut id=  lines.next().expect("Could not parse for the ID line").to_string();
        let     seq= lines.next().expect("Could not parse for the seq line");
                     lines.next().expect("Could not parse for the + line");
        let     qual=lines.next().expect("Could not parse for the qual line");
        
        // Edit the ID by adding the count 
        id.push_str(" collapsed_reads:");
        id.push_str(&count.to_string());

        // Print the sequence and edit the qual
        println!("{}\n{}\n+\n{}",id,seq,recalculate_qual(qual,count));
        // Paired end
        if lines_per_read==8 {
            let mut id2=  lines.next().expect("Could not parse for the R2 ID line").to_string();
            let     seq2= lines.next().expect("Could not parse for the R2 seq line");
                         lines.next().expect("Could not parse for the R2 + line");
            let     qual2=lines.next().expect("Could not parse for the R2 qual line");
            
            // Edit the ID by adding the count 
            id2.push_str(" collapsed_reads:");
            id2.push_str(&count.to_string());

            // Print the sequence and edit the qual
            println!("{}\n{}\n+\n{}",id2,seq2,recalculate_qual(qual2,count));
        }
    }
}
*/

fn recalculate_qual(qual_str: &str, count: u32) -> String {
    let mut qual_out = String::new();

    let max_qual = 'I' as u8;
    let min_qual = '#' as u8;

    let qual = qual_str.to_string();
    for qual_char in qual.chars() {
        let qual_int = qual_char as u8 as f32 - 33.0;
        //let ten:f32=10.0;
        let p :f32 = TEN.powf((-1.0 * qual_int)/TEN);
        let p_recalc :f32 = p.powi(count as i32);
        let mut qual_recalc :f32 = -TEN * (p_recalc).log(TEN)+33.0;
        // check for overflow before switching to u8
        if qual_recalc.is_infinite() || qual_recalc > 255.0 {
            qual_recalc = 255.0;
        }

        // switch to u8 and then the corresponding char
        let mut qual_recalc_char = qual_recalc.floor() as u8 as char;
        if (qual_recalc_char as u8) > max_qual {
            qual_recalc_char = 'I';
        }
        // a reduction in quality is not expected... but just in case.
        if (qual_recalc_char as u8) < min_qual {
            qual_recalc_char = '#';
        }
        qual_out.push(qual_recalc_char);
    }

    return qual_out;
}





