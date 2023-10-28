//! Blunt-end trims using 0-based coordinates
//! 
//! # Examples
//! 
//! ## Trim five bases from the right side
//! ```bash
//! cat file.fastq | fasten_trim -l -5 > trimmed.fastq
//! ```
//!
//! ## Keep a maximum of 100bp
//! ```bash
//! cat file.fastq | fasten_trim -l 99 > trimmed.fastq
//! ```
//! ## Trim 5bp from the left side
//! ```bash
//! cat file.fastq | fasten_trim -f 4  > trimmed.fastq
//! ```
//! 
//! # Usage
//! 
//! ```text
//! Usage: fasten_trim [-h] [-n INT] [-p] [-v] [-f INT] [-l INT]
//! 
//! Options:
//!     -h, --help          Print this help menu.
//!     -n, --numcpus INT   Number of CPUs (default: 1)
//!     -p, --paired-end    The input reads are interleaved paired-end
//!     -v, --verbose       Print more status messages
//!     -f, --first-base INT
//!                         The first base to keep (default: 0)
//!     -l, --last-base INT The last base to keep. If negative, counts from the
//!                         right. (default: 0)
//! ```

extern crate fasten;
extern crate statistical;
extern crate getopts;
extern crate threadpool;

use std::fs::File;
use std::io::BufReader;
use std::cmp::min;

use fasten::fasten_base_options;
use fasten::fasten_base_options_matches;
use fasten::logmsg;
use fasten::io::fastq;
use fasten::io::seq::Seq;

fn main(){
    let mut opts = fasten_base_options();

    // script-specific options
    opts.optopt("f","first-base","The first base to keep (default: 0)","INT");
    opts.optopt("l","last-base","The last base to keep (default: 0)","INT");

    let matches = fasten_base_options_matches("Blunt-end trims using 0-based coordinates", opts);

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

    let _num_cpus:usize = {
      if matches.opt_present("numcpus") {
        /*
        matches.opt_str("numcpus")
            .expect("ERROR: could not understand parameter --numcpus")
            .parse()
            .expect("ERROR: --numcpus is not an INT");
        */
        logmsg("Warning: multithreading this script currently slows it down. Resetting to 1 cpu.  Avoid this warning by not using --numcpus");
        1 as usize
      } else {
        1 as usize
      }
    };
    
    // Read from stdin
    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let fastq_reader = fastq::FastqReader::new(my_buffer);
    let fastq_iter  = fastq_reader.into_iter();
    for seq in fastq_iter {

        let trimmed:String = trim_worker(seq, first_base, last_base);
        println!("{}", trimmed);
    }
}

/// Trim a set of fastq entries and send it to a channel
fn trim_worker(seq:Seq, first_base:usize, last_base:usize ) -> String {

    // The last position is either the last_base parameter
    // or the last position in the string, whichever is less.
    let last_base_tmp = match last_base {
        // But if the position is not specified, then it is the seq length
        0 => {
            // zero based
            seq.seq.len()-1
        },
        _ => {
            min(seq.seq.len()-1, last_base)
        }
    };

    let sequence = &seq.seq[first_base..last_base_tmp];
    let quality  = &seq.qual[first_base..last_base_tmp];

    let trimmed = format!("{}\n{}\n+\n{}", seq.id, sequence, quality);
    return trimmed;
}
  

