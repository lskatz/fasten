extern crate fasten;
extern crate statistical;
extern crate getopts;
extern crate threadpool;

use std::fs::File;
use std::io::BufReader;
use std::env;
use std::cmp::min;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;

use fasten::fasten_base_options;
use fasten::logmsg;
use fasten::io::fastq;
//use fasten::io::seq::Cleanable;
use fasten::io::seq::Seq;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = fasten_base_options();

    // script-specific options
    opts.optopt("f","first-base","The first base to keep (default: 0)","INT");
    opts.optopt("l","last-base","The last base to keep. If negative, counts from the right. (default: 0)","INT");

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");

    if matches.opt_present("help") {
        println!("Blunt-end trims using 0-based coordinates\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let (tx, rx):(std::sync::mpsc::Sender<String>,std::sync::mpsc::Receiver<String>) = channel();

    let paired_end:bool = matches.opt_present("paired-end");

    let first_base:usize ={
        if matches.opt_present("first-base") {
            matches.opt_str("first-base")
                .expect("ERROR: could not understand parameter --first-base")
                .parse()
                .expect("ERROR: --first-base is not an INT")
        } else {
            0
        }
    };

    let last_base:usize ={
        if matches.opt_present("last-base") {
            matches.opt_str("last-base")
                .expect("ERROR: could not understand parameter --last-base")
                .parse()
                .expect("ERROR: --last-base is not an INT")
        } else {
            0
        }
    };

    let num_cpus:usize = {
      if matches.opt_present("numcpus") {
        logmsg("Warning: multithreading this script currently slows it down");
        matches.opt_str("numcpus")
            .expect("ERROR: could not understand parameter --numcpus")
            .parse()
            .expect("ERROR: --numcpus is not an INT")
      } else {
          1
      }
    };
    
    /*
     * Set up multithreading. Each thread will get 100k
     * reads at a time.
     */
    let pool = ThreadPool::with_name("worker".into(), num_cpus);

    // Read from stdin
    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let fastq_reader = fastq::FastqReader::new(my_buffer);
    let mut fastq_iter   = fastq_reader.into_iter();
    while let Some(seq) = fastq_iter.next() {
        // start off an array of seq objects that will 
        // contain R1 (and R2).
        let mut seqs:Vec<Seq> = vec![seq];
        if paired_end {
          seqs.push(
            fastq_iter.next()
              .expect("Tried to get the second sequence in a pair but ran into an error")
          );
        }

        // Send this single end or paired end to the queue
        let tx2 = tx.clone();
        pool.execute(move|| {
          trim_worker(seqs, first_base, last_base, tx2);
        });
    }

    pool.join();
    drop(tx); // disconnects the channel

    let receiver = rx.iter();
    for entry in receiver {
      println!("{}",entry);
    }

}

fn trim_worker(seqs:Vec<Seq>, first_base:usize, last_base:usize, tx:std::sync::mpsc::Sender<String> ){

  for seq in seqs{
    // The last position is either the last_base parameter
    // or the last position in the string, whichever is less.
    let last_base_tmp = min(seq.seq.len(), last_base);

    let sequence = &seq.seq[first_base..last_base_tmp];
    let quality  = &seq.qual[first_base..last_base_tmp];

    let trimmed = format!("{}\n{}\n+\n{}", seq.id, sequence, quality);
    tx.send(trimmed).unwrap();
  }
}
  

/*
fn trim_worker_old(sub_lines_buffer:&mut Vec<String>, first_base:usize, last_base:usize, tx:std::sync::mpsc::Sender<String> ){
  let this_thread = thread::current();
  let _tid = this_thread.id(); // for debugging

  sub_lines_buffer.reverse();
  
  while sub_lines_buffer.len() > 0 {
    //let mut entry_splice = &sub_lines_buffer.splice(0..4, vec![]);
    //let entry = vec![entry_splice];
    let id       = sub_lines_buffer.pop().unwrap();
    let mut seq  = sub_lines_buffer.pop().unwrap();
    let plus     = sub_lines_buffer.pop().unwrap();
    let mut qual = sub_lines_buffer.pop().unwrap();

    let last_base_tmp = min(seq.len(), last_base);
    seq  = String::from(seq);
    qual = String::from(qual);

    let entry = format!("{}\n{}\n{}\n{}",
      id, &seq[first_base..last_base_tmp], plus, &qual[first_base..last_base_tmp]
    );

    tx.send(entry).unwrap();

  }
  //eprintln!("{:?} finished {}", &_tid, &num_lines);
}

*/

