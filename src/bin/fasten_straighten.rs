//! Convert a fastq file to a standard 4-lines-per-entry format
//! 
//! # Examples
//! 
//! ```bash
//! cat weird.fastq | fasten_straighten > four-per-entry.fastq
//! ```
//! 
//! # Usage
//! 
//! ```text
//! Usage: fasten_straighten [-h] [-n INT] [-p] [-v]
//! 
//! Options:
//!     -h, --help          Print this help menu.
//!     -n, --numcpus INT   Number of CPUs (default: 1)
//!     -p, --paired-end    The input reads are interleaved paired-end
//!     -v, --verbose       Print more status messages
//! ```

extern crate getopts;
extern crate fasten;
use std::fs::File;
use std::io::BufReader;

use fasten::fasten_base_options;
use fasten::io::fastq;
use fasten::io::seq::Cleanable;
use fasten::logmsg;

use std::env;

#[test]
/// Test to see whether we read the challenge dataset correctly
fn challenge_dataset () {
    // Open the difficult file
    let challenge_file = File::open("testdata/four_reads.gt_16_lines.fastq").expect("Could not open testdata/four_reads.gt_16_lines.fastq");
    let challenge_buffer=BufReader::new(challenge_file);
    let challenge_reader=fastq::FastqReader::new_careful(challenge_buffer);
    let mut challenge_string = String::new();
    for seq_obj in challenge_reader {
        challenge_string.push_str(&seq_obj.to_string());
    }

    // Open the easy file
    let easy_file  = File::open("testdata/four_reads.fastq").expect("Could not open testdata/four_reads.fastq");
    let easy_buffer= BufReader::new(easy_file);
    let easy_reader=fastq::FastqReader::new(easy_buffer);
    let mut easy_string = String::new();
    for seq_obj in easy_reader {
        easy_string.push_str(&seq_obj.to_string());
    }
    
    assert_eq!(challenge_string,easy_string);
}


fn main(){
    let args: Vec<String> = env::args().collect();
    let opts = fasten_base_options();
    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");
    if matches.opt_present("help") {
        println!("Convert a fastq file to a standard 4-lines-per-entry format\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }
    if matches.opt_present("paired-end") {
        logmsg("WARNING: --paired-end is not utilized in this script");
    }

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let fastq_reader=fastq::FastqReader::new_careful(my_buffer);
    for seq in fastq_reader {
        seq.print();
    }
}

