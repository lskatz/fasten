//! Filter reads based on a regular expression.
//! 
//! # Examples
//! 
//! ## Find a specific read
//! ```bash
//! cat file.fastq | fasten_regex --which ID --regex 'my-specific-read-id-1234' > my_read.fastq
//! ```
//!
//! ## Find a specific read but also keep its pair
//! ```bash
//! cat file.fastq | fasten_regex --which ID --regex 'my-specific-read-id-1234' --paired-end > my_pairs.fastq
//! ```
//!
//! ## Find a specific motif
//! ```bash
//! cat file.fastq | fasten_regex --which SEQ --regex ATAT > atat-motif.fastq
//! ```
//! 
//! # Usage
//! 
//! ```text
//! Usage: fasten_regex [-h] [-n INT] [-p] [-v] [-r STRING] [-w String]
//! 
//! Options:
//!     -h, --help          Print this help menu.
//!     -n, --numcpus INT   Number of CPUs (default: 1)
//!     -p, --paired-end    The input reads are interleaved paired-end
//!     -v, --verbose       Print more status messages
//!     -r, --regex STRING  Regular expression (default: '.')
//!     -w, --which String  Which field to match on? ID, SEQ, QUAL. Default: SEQ
//! ```
extern crate getopts;
extern crate fasten;
extern crate regex;
extern crate threadpool;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use regex::Regex;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

use fasten::fasten_base_options;
use fasten::fasten_base_options_matches;

fn main(){
    let mut opts = fasten_base_options();
    // Options specific to this script
    opts.optopt("r","regex","Regular expression (default: '.')","STRING");
    opts.optopt("w","which","Which field to match on? ID, SEQ, QUAL. Default: SEQ","String");
    //opts.optflag("e","exclude","Exclude these reads instead of including them");

    let matches = fasten_base_options_matches("Filter reads based on a regular expression.", opts);

    let (tx, rx):(std::sync::mpsc::Sender<String>,std::sync::mpsc::Receiver<String>) = channel();

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
        let tx2          = tx.clone();
        pool.execute(move|| {
          let regex = Regex::new(&regex_param2)
              .expect("malformed seq regex within thread, given by --regex");
          // Print if it's a match
          let should_print:bool = match the_field.as_str(){
            "SEQ" => {
              if regex.is_match(&all_seq) {
                true
              } else {
                false
              }
            }, 
            "ID" => {
              if regex.is_match(&all_seq) {
                true
              } else {
                false
              }
            },
            "QUAL" => {
              if regex.is_match(&all_seq) {
                true
              } else {
                false
              }
            }
            _ => {
              panic!("{} is not a valid key to match on", &the_field);
            }
          };

                
          if should_print {
            if is_paired_end {
              tx2.send(format!("{}\n{}\n+\n{}\n{}\n{}\n+\n{}",
                id, seq, qual,
                id2,seq2,qual2
              )).unwrap();
            } else {
              tx2.send(format!("{}\n{}\n+\n{}",
                id, seq, qual
              )).unwrap();
            }
          }
        });
    }
    pool.join();
    drop(tx); // disconnects the channel

    // TODO why not make this a separate thread
    let receiver = rx.iter();
    for entry in receiver {
      println!("{}",entry);
    }
}

