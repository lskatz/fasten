//! Validates your reads and makes you feel good about yourself!
//! 
//! # Examples
//! 
//! ## Quick validation with stderr message
//! ```bash
//! cat file.fastq | fasten_validate --verbose
//! ```
//!
//! ## Validate that your reads are paired end
//! ```bash
//! cat R1.fastq R2.fastq | fasten_shuffle | fasten_validate --paired-end
//! ```
//!
//! ## Parallelize
//! Large-scale validation of PE reads
//! with 4 CPUs and xargs
//! ```bash
//! \ls *_1.fastq.gz | xargs -n 1 -P 4 bash -c '
//!   echo -n "." >&2 # progress bar
//!   R1=$0
//!   R2=${0/_1.fastq.gz/_2.fastq.gz}
//!   zcat $R1 $R2 | fasten_shuffle | fasten_validate --paired-end
//! '
//! ```
//! 
//! # Usage
//! 
//! ```text
//!     Usage: fasten_validate [-h] [-n INT] [-p] [-v] [--min-length INT] [--min-quality FLOAT] [--paired-end] [--print-reads] [-v]
//!     
//!     Options:
//!         -h, --help          Print this help menu.
//!         -n, --numcpus INT   Number of CPUs (default: 1)
//!         -p, --paired-end    The input reads are interleaved paired-end
//!         -v, --verbose       Print more status messages
//!             --min-length INT
//!                             Minimum read length allowed
//!             --min-quality FLOAT
//!                             Minimum quality allowed
//!             --paired-end    The reads are interleaved paired-end
//!             --print-reads   Print the reads as they are being validated (useful
//!                             for unix pipes)
//! ```

extern crate getopts;
extern crate fasten;
extern crate regex;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use regex::Regex;

use fasten::fasten_base_options;
use fasten::fasten_base_options_matches;

fn main(){
    let mut opts = fasten_base_options();
    // Options specific to this script
    opts.optopt("","min-length","Minimum read length allowed","INT");
    opts.optopt("","min-quality","Minimum quality allowed","FLOAT");
    opts.optflag("","paired-end","The reads are interleaved paired-end");
    opts.optflag("","print-reads","Print the reads as they are being validated (useful for unix pipes)");

    let matches = fasten_base_options_matches("Validates your reads and makes you feel good about yourself!", opts);

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);

    let lines_per_read={
        if matches.opt_present("paired-end") {
            8
        }else{
            4
        }
    };
    let min_length :usize={
        if matches.opt_present("min-length") {
            matches.opt_str("min-length")
                .expect("ERROR parsing min-length")
                .parse()
                .expect("ERROR parsing min-length as INT")
        } else {
            0
        }
    };
    let min_qual :f32={
        if matches.opt_present("min-quality") {
            matches.opt_str("min-quality")
                .expect("ERROR parsing min-quality")
                .parse()
                .expect("ERROR parsing min-quality as FLOAT")
        } else {
            0.0
        }
    };


    // save this option to avoid the overhead of calling
    // opt_present many times in a loop
    let should_print=matches.opt_present("print-reads");

    // If there is a match on these, then mark invalid.
    // In other words, we are looking for a pattern that
    // is NOT the target seq or qual
    let seq_regex = Regex::new(r"[^a-zA-Z]").expect("malformed seq regex");
    //let qual_regex= Regex::new(r"[^!-Za-z]").expect("malformed qual regex");
    let qual_regex= Regex::new(r"\s").expect("malformed qual regex");

    // TODO have a print buffer, something like 4096 bytes

    let mut i = 0;
    for line in my_buffer.lines() {
        let line=line.expect("ERROR: did not get a line");
        if should_print {
            println!("{}",line);
        }

        // TODO pattern match for each kind of line:
        // id, seq, +, qual
        match i%4{
            0=>{
                if line.chars().nth(0).unwrap() != '@' {
                    panic!("ERROR: first character of the identifier is not @ in the line {}. Contents are:\n  {}",i,line); 
                }
            }
            1=>{
                if seq_regex.is_match(&line) {
                    panic!("ERROR: there are characters that are not in the alphabet in line {}. Contents are:\n  {}",i,line);
                }
                if min_length > 0 && line.len() > min_length {
                    panic!("ERROR: sequence at line {} is less than the minimum sequence length",i);
                }
            }
            2=>{
                if line.chars().nth(0).unwrap() != '+' {
                    panic!("ERROR: first character of the qual identifier is not + in the line {}. Contents are:\n  {}",i,line); 
                }
            }
            3=>{
                if qual_regex.is_match(&line) {
                    for cap in qual_regex.captures_iter(&line) {
                        eprintln!("Illegal qual character found: {}", &cap[0]);
                    }
                    panic!("ERROR: there are characters that are not qual characters in line {}. Contents are:\n  {}",i,line);
                }
                // only calculate read quality if we are testing for it
                if min_qual > 0.0 {
                    let mut qual_total :usize = 0;
                    for q in line.chars() {
                        qual_total += q as usize;
                    }
                    let avg_qual :f32 = qual_total as f32 / line.len() as f32 - 33.0;
                    if avg_qual < min_qual {
                        panic!("ERROR: quality is less than min qual in line {}.\n  Avg qual is {}.\n  Min qual is {}\n  Contents are:\n  {}",i,avg_qual,min_qual,line);
                    }
                }
            }
            _=>{
                panic!("INTERNAL ERROR");
            }
        }
        i += 1;
    }
    if i % lines_per_read > 0{
        panic!("ERROR: incomplete fastq entry. Num lines: {}",i);
    }

    if matches.opt_present("verbose") {
        fasten::logmsg("These reads have been validated!");
    }
}


