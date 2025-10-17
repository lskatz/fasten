//! get the first N reads or bases
//! 
//! # Examples
//!
//! Example 1: Get the first 10 reads
//! 
// ```bash
//! fasten_head -r 10 < in.fq > out.fq
//! ```
//!
//! Example 2: Get the first 100 bases
//! Note: the last read will not be truncated and so you may have more than 100 bases
//! 
// ```bash
//! fasten_head -b 100 < in.fq > out.fq
//! ```
//!
//! # Usage
//! 
//! ```text
//! Usage: fasten_head [-h] [-n INT] [-p] [--verbose] [--version] [-r INT] [-b INT]
//! Options:
//!     -h, --help          Print this help menu.
//!     -n, --numcpus INT   Number of CPUs (default: 1)
//!     -p, --paired-end    The input reads are interleaved paired-end
//!         --verbose       Print more status messages
//!         --version       Print the version of Fasten and exit
//!     -r, --reads INT     Number of reads or pairs of reads to keep, default: 10
//!     -b, --bases INT     Number of bases to keep, default: 0 (zero for no
//!                         limit). If bases are reached in the middle of a read,
//!                         the complete read will still be printed.
//! ```
//!

extern crate fasten;
extern crate getopts;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use fasten::fasten_base_options;
use fasten::fasten_base_options_matches;

fn main(){
    let mut opts = fasten_base_options();

    opts.optopt("r","reads","Number of reads or pairs of reads to keep, default: 10","INT");
    opts.optopt("b","bases","Number of bases to keep, default: 0 (zero for no limit). If bases are reached in the middle of a read, the complete read will still be printed.","INT");

    let matches = fasten_base_options_matches("Keep first N reads or bases", opts);

    let max_reads: usize = {
        if matches.opt_present("reads") {
            matches.opt_str("reads")
            .expect("ERROR parsing reads parameter")
            .parse()
            .expect("ERROR parsing reads as an integer")
        } else {
            10
        }
    };

    let max_bases: usize = {
        if matches.opt_present("bases") {
            matches.opt_str("bases")
            .expect("ERROR parsing bases parameter")
            .parse()
            .expect("ERROR parsing bases as an integer")
        } else {
            0
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
    let mut bases_counter=0;
    let mut reads_counter=0;
    for line in my_buffer.lines() {
        // unwrap the line here and shadow-set the variable.
        let line=line.expect("ERROR: did not get a line");
        line_counter+=1;
        entry.push_str(&line);
        entry.push_str("\n");

        // keep track of number of bases if needed
        if line_counter % lines_per_read == 2 {
            // we are in the sequence line
            bases_counter+=line.len();
        }

        // Action if we have a full entry when mod 0
        if line_counter % lines_per_read == 0 {
            // keep track of how many reads or pairs of reads
            reads_counter+=1;

            // check if we have reached any maximum
            if max_bases > 0 && bases_counter >= max_bases {
                // we have reached the maximum number of bases
                break;
            }
            if reads_counter > max_reads {
                // we have reached the maximum number of reads
                break;
            }

            // print the entry
            print!("{}",entry);

            // reset the entry string
            entry = String::new();
        }
    }
}


