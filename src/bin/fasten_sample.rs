//! downsample your reads
//! 
//! # Examples
//! 
//! ## Get 10% of the reads
//! ```bash
//! cat file.fastq | fasten_sample --frequency 0.1 > out.fastq
//! ```
//! 
//! # Usage
//! 
//! ```text
//!     Usage: fasten_sample [-h] [-n INT] [-p] [-v] [-f FLOAT]
//!     
//!     Options:
//!         -h, --help          Print this help menu.
//!         -n, --numcpus INT   Number of CPUs (default: 1)
//!         -p, --paired-end    The input reads are interleaved paired-end
//!         -v, --verbose       Print more status messages
//!         -f, --frequency FLOAT
//!                             Frequency of sequences to print, 0 to 1. Default: 1
//! ```

extern crate fasten;
extern crate getopts;
extern crate rand;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use rand::prelude::*;

use fasten::fasten_base_options;
use fasten::fasten_base_options_matches;

fn main(){
    let mut opts = fasten_base_options();

    opts.optopt("f","frequency","Frequency of sequences to print, 0 to 1. Default: 1","FLOAT");

    let matches = fasten_base_options_matches("Downsample your reads", opts);

    let frequency :f32 = {
        if matches.opt_present("frequency") {
            matches.opt_str("frequency")
            .expect("ERROR parsing frequency parameter")
            .parse()
            .expect("ERROR parsing frequency as a float")
        } else {
            1.0
        }
    };

    let lines_per_read={
        if matches.opt_present("paired-end") {
            8
        }else{
            4
        }
    };

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut line_counter =0;
    let mut entry = String::new();
    let mut randoms :Vec<f32> = vec![];
    for line in my_buffer.lines() {
        // unwrap the line here and shadow-set the variable.
        let line=line.expect("ERROR: did not get a line");
        line_counter+=1;
        entry.push_str(&line);
        entry.push_str("\n");

        // Action if we have a full entry when mod 0
        if line_counter % lines_per_read == 0 {
            if randoms.len() < 1 {
                randoms = rand_32();
            }
            let r:f32 = randoms.pop().unwrap();
            // Should we print?
            if r < frequency {
                print!("{}",entry);
            }
            // reset the entry string
            entry = String::new();
        }
    }
}

/// Generate a set of random floats
fn rand_32() -> Vec<f32> {
  let mut rng = rand::thread_rng();

  let floats :Vec<f32> = (0..10000)
      .map(|_| rng.gen::<f32>())
      .collect();

  return floats;
}

