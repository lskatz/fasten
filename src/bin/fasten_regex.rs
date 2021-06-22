extern crate getopts;
extern crate fasten;
extern crate regex;
extern crate threadpool;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use regex::Regex;
use threadpool::ThreadPool;
//use std::sync::mpsc;

use fasten::fasten_base_options;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = fasten_base_options();
    // Options specific to this script
    opts.optopt("r","regex","Regular expression (default: '.')","STRING");
    opts.optopt("w","which","Which field to match on? ID, SEQ, QUAL. Default: SEQ","String");
    //opts.optflag("e","exclude","Exclude these reads instead of including them");

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");
    if matches.opt_present("help") {
        println!("Filter reads based on a regular expression.\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);

    let is_paired_end=matches.opt_present("paired-end");

    let which_field:String={
        if matches.opt_present("which") {
            matches.opt_str("which").expect("ERROR parsing --which")
                .to_uppercase()
        } else {
            "SEQ".to_string()
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

    // by default the regex parameter will be "." for "everything"
    let regex_param={
        if matches.opt_present("regex") {
            matches.opt_str("regex")
                .expect("ERROR: could not parse regex parameter")
        } else {
            String::from(".")
        }
    };

    // Error checking the regex in the main thread
    let _regex = Regex::new(&regex_param)
        .expect("malformed seq regex given by --regex");

    if matches.opt_present("verbose") {
        eprintln!("Regular expression: {}",regex_param);
    }

    let pool = ThreadPool::new(num_cpus);

    let mut buffer_iter = my_buffer.lines();

    while let Some(line) = buffer_iter.next() {
        let id  = line.expect("ERROR reading the ID line");
        let seq = buffer_iter.next().expect("ERROR reading a sequence line")
            .expect("ERROR reading a sequence line");
                  buffer_iter.next().expect("ERROR reading a plus line")
                      .expect("ERROR reading the plus line");
        let qual= buffer_iter.next().expect("ERROR reading a qual line")
            .expect("ERROR reading a qual line");

        // Things to regex-match on
        let mut all_id    =id.clone();
        let mut all_seq   =seq.clone();
        let mut all_qual  =qual.clone();

        let mut id2   =String::new();
        let mut seq2  =String::new();
        let mut qual2 =String::new();

        // Get R2
        if is_paired_end {
            id2  = buffer_iter.next().expect("ERROR reading the ID line")
                .expect("ERROR reading the ID line");
            seq2 = buffer_iter.next().expect("ERROR reading a sequence line")
                .expect("ERROR reading a sequence line");
                       buffer_iter.next().expect("ERROR reading a plus line")
                          .expect("ERROR reading the plus line");
            qual2= buffer_iter.next().expect("ERROR reading a qual line")
                .expect("ERROR reading a qual line");

            all_id.push_str(&id2);
            all_seq.push_str(&seq2);
            all_qual.push_str(&qual2);
        }

        // Copy some things to the threads
        let the_field    = String::from(&which_field);
        let regex_param2 = regex_param.clone();
        pool.execute(move|| {
          let regex = Regex::new(&regex_param2)
              .expect("malformed seq regex within thread, given by --regex");
          // Print if it's a match
          if (&the_field == "SEQ"  && regex.is_match(&all_seq))
           ||(&the_field == "ID"   && regex.is_match(&all_id))
           ||(&the_field == "QUAL" && regex.is_match(&all_qual)) {
              println!("{}\n{}\n+\n{}",id,seq,qual);
              if is_paired_end { 
                  println!("{}\n{}\n+\n{}",id2,seq2,qual2);
              }
          }
        });
    }
}
