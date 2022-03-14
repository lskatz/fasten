//! Transforms any low-quality base to 'N'
//! 
//! # Examples
//! 
//! ```bash
//! cat file.fastq | fasten_quality_filter > file_with_Ns.fastq
//! ```
//! 
//! ```text
//! ## Usage
//! 
//! Usage: fasten_quality_filter [-h] [-n INT] [-p] [-v] [-m INT]
//! 
//! Options:
//!     -h, --help          Print this help menu.
//!     -n, --numcpus INT   Number of CPUs (default: 1)
//!     -p, --paired-end    The input reads are interleaved paired-end
//!     -v, --verbose       Print more status messages
//!     -m, --max-quality INT
//!                         The maximum quality at which a base will be
//!                         transformed to 'N'
//! ```    
extern crate fasten;
extern crate statistical;
extern crate getopts;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use fasten::fasten_base_options;
use fasten::logmsg;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = fasten_base_options();

    // script-specific options
    opts.optopt("m","max-quality","The maximum quality at which a base will be transformed to 'N'","INT");

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");

    if matches.opt_present("help") {
        println!("Transforms any low-quality base to 'N'\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }
    if matches.opt_present("paired-end") {
        logmsg("WARNING: --paired-end is not utilized in this script");
    }

    let max_quality_offset :u8={
        if matches.opt_present("max-quality") {
            matches.opt_str("max-quality")
                .expect("ERROR: could not understand parameter --max-quality")
                .parse()
                .expect("ERROR: --max-quality is not an INT")
        } else {
           0 
        }
    };
    let max_quality :u8 = max_quality_offset + 33;


    let filename = "/dev/stdin";
    
    // read the file
    let my_file = File::open(&filename).expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut num_lines=0;

    let mut current_id :String = String::new();
    let mut current_seq :String = String::new();
    for line in my_buffer.lines() {
        num_lines+=1;

        match num_lines % 4 {
            1 => {
                current_id = line.expect("ERROR reading id line");
            }
            2 => {
                current_seq = line.expect("ERROR reading seq line");
            }
            0 => {
                let current_qual_cigar = line.expect("ERROR reading qual line");
                let mut seq_chars = current_seq.chars();
                let mut new_seq =String::new();
                let mut new_qual=String::new();
                for current_qual in current_qual_cigar.chars() {
                    let current_nt=seq_chars.next().expect("ERROR: could not get the next nt in the sequence");
                    let phred = current_qual as u8;
                    if phred < max_quality {
                        new_seq.push('N');
                        new_qual.push('!');
                    } else {
                        new_seq.push(current_nt);
                        new_qual.push(current_qual);
                    }
                }

                println!("{}\n{}\n+\n{}",&current_id,new_seq,new_qual);
            }
            _ => { }
        };
    }
}

