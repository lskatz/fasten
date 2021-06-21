extern crate fasten;
extern crate statistical;
extern crate getopts;
extern crate threadpool;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::thread;
use std::cmp::min;
use threadpool::ThreadPool;

use fasten::fasten_base_options;
use fasten::logmsg;

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
    if matches.opt_present("paired-end") {
        logmsg("WARNING: --paired-end is not utilized in this script");
    }

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
    let filename = "/dev/stdin";
    
    // read the file
    let my_file = File::open(&filename).expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut num_lines=0;
    let mut lines_buffer:Vec<String> = vec![];

    for line_result in my_buffer.lines() {
        num_lines+=1;

        let line = line_result.expect("Error reading line");
        &lines_buffer.push(line);

        if num_lines % 888888 == 0 {
          eprintln!("{} lines queued", &num_lines);
          // grab the buffer into a buffer for the thread
          // and then empty the main buffer
          let sub_lines_buffer = lines_buffer.clone();
          lines_buffer = vec![];
          pool.execute(move|| {
            logmsg("enqueuing");
            trim_worker(&sub_lines_buffer, first_base, last_base);
          });
        }
    }
    // One last time with the remaining buffer.
    // Need to queue it so that it doesn't make the main 
    // thread do extra work.
    pool.execute(move|| {
      trim_worker(&lines_buffer, first_base, last_base);
    });

    pool.join();
}

fn trim_worker(sub_lines_buffer:&Vec<String>, first_base:usize, last_base:usize ){
  let this_thread = thread::current();
  let _tid = this_thread.id(); // for debugging
  let mut num_lines:u32 = 0;

  //sub_lines_buffer.into_inter().map(|x|{
  for subline in sub_lines_buffer {
    num_lines+=1;

    // Every other line gets trimmed
    match num_lines % 2 {
        0 => {
            let seq_or_qual = &subline;
            let last_base_tmp = min(seq_or_qual.len(), last_base);
            println!("{}",&seq_or_qual[first_base..last_base_tmp]);
        }
        _ => {
            //println!("{} {:?}",&subline, tid);
            println!("{}",&subline);
        }
    };
  }
  //eprintln!("{:?} finished {}", &_tid, &num_lines);
}


