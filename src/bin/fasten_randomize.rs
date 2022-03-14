//! Create random reads from stdin.
//! 
//! # Examples
//! 
//! ```bash
//! # General usage to randomize the order of the reads
//! cat file.fastq | fasten_randomize > random.fastq
//! # Get one random read. Entries will always be in a 4-line format.
//! cat file.fastq | fasten_randomize | head -n 4 > one_read.fastq
//! # keep the paired ends together if paired
//! cat R1.fastq R2.fastq | fasten_shuffle | fasten_randomize --paired-end | head -n 8 > one_pair.fastq
//! ```
//! 
//! ```text
//! # Usage
//! 
//! Usage: fasten_randomize [-h] [-n INT] [-p] [-v]
//! 
//! Options:
//!     -h, --help          Print this help menu.
//!     -n, --numcpus INT   Number of CPUs (default: 1)
//!     -p, --paired-end    The input reads are interleaved paired-end
//!     -v, --verbose       Print more status messages
//! ```
extern crate getopts;
extern crate fasten;
extern crate rand;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use rand::{Rng,thread_rng};

use fasten::fasten_base_options;

fn main(){
    let args: Vec<String> = env::args().collect();
    let opts = fasten_base_options();
    //script-specific flags

    let matches = opts.parse(&args[1..]).expect("Error: could not parse parameters");
    if matches.opt_present("help") {
        println!("Create random reads from stdin.\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let lines_per_read :u32={
        if matches.opt_present("paired-end") {
            8
        }else{
            4
        }
    };

    print_reads_from_stdin(lines_per_read);
}

fn print_reads_from_stdin(lines_per_read :u32) -> () {
    // Start off with a capacity of 100k reads.
    let mut seqs :Vec<String> = Vec::with_capacity(100000);
    let my_file = File::open("/dev/stdin").expect("Could not open stdin");
    let my_buffer=BufReader::new(my_file);
    let mut lines = my_buffer.lines();
    while let Some(id) = lines.next() {
        let mut entry = id.expect("ERROR: could not parse the ID line");
        for _ in 1..lines_per_read {
            entry.push('\n');
            let next_line = lines.next()
                .expect("ERROR: could not get the next line")
                .expect("ERROR: could not parse the next line");
            entry.push_str(&next_line);
        }

        seqs.push(entry);
    }

    // choose random reads
    let mut rng = thread_rng();
    rng.shuffle(&mut seqs);
    for seq in seqs {
        println!("{}",seq);
    }
}


