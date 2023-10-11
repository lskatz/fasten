//! Marks up your reads with useful information like read length
//! 
//! # Examples
//! 
//! ## Quick validation with stderr message
//! ```bash
//! cat file.fastq | fasten_validate > markedup.fastq
//! cat file.fastq | fasten_validate --paired-end > markedup-paired.fastq
//! ```
//!
//! # Usage
//! 
//! ```text
//! Usage: fasten_validate [-h] [-n INT] [-p] [--verbose] [--version] [--paired-end]
//!
//! Options:
//!     -h, --help          Print this help menu.
//!     -n, --numcpus INT   Number of CPUs (default: 1)
//!     -p, --paired-end    The input reads are interleaved paired-end
//!         --verbose       Print more status messages
//!         --version       Print the version of Fasten and exit
//! ```
//!
//! The fields will be found on the defline of the sequence and include:
//!
//! | key | type | example | note |
//! | --- | ---   | ---  | -----  |
//! | id-at | boolean (1 or 0) | id-at:1 | | 
//! | seq-invalid-chars | string | seq-invalid-chars:'$$%' | |
//! | qual-invalid-chars | string | qual-invalid-chars:'[]' | |
//! | seq-length | int | seq-length:100 | |
//! | id-plus | boolean | id-plus:1 | Whether or not the plus sign was found on 3rd line |
//! | avg-qual | float | avg-qual:17.52 | |
//! | qual-length | int | qual-length:100 | Length of the quality score line |
//!

// TODO add points that were validated into the sequence deflines: length, is-paired, seq-regex=1, and anything else

extern crate getopts;
extern crate fasten;
extern crate regex;
use std::fs::File;
//use std::io::BufReader;
use std::io::{BufRead,BufReader};

use regex::Regex;

use fasten::fasten_base_options;
use fasten::fasten_base_options_matches;

fn main(){
    let opts = fasten_base_options();
    // Options specific to this script
    // opts.optflag("","paired-end","The reads are interleaved paired-end");

    let matches = fasten_base_options_matches("Marks up your reads with useful information like read length", opts);

    let lines_per_read :u8 ={
        if matches.opt_present("paired-end") {
            8
        }else{
            4
        }
    };
    
    // If there is a match on these, then mark invalid.
    // In other words, we are looking for a pattern that
    // is NOT the target seq or qual
    let seq_regex = Regex::new(r"[^a-zA-Z]").expect("malformed seq regex");
    //let qual_regex= Regex::new(r"[^!-Za-z]").expect("malformed qual regex");
    let qual_regex= Regex::new(r"\s").expect("malformed qual regex");

    validate_reads(lines_per_read, seq_regex, qual_regex);

    if matches.opt_present("verbose") {
        fasten::logmsg("These reads have been validated!");
    }
}

fn validate_reads(lines_per_read: u8, seq_regex: regex::Regex, qual_regex: regex::Regex) {
    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let mut my_buffer = BufReader::new(my_file);

    let mut id   = String::new();
    let mut seq  = String::new();
    let mut plus = String::new();
    let mut qual = String::new();

    let mut i = 0;
    loop{

        id.clear();
        if my_buffer.read_line(&mut id).expect("Cannot read new line") == 0 {
            break;
        }

        seq.clear();
        my_buffer.read_line(&mut seq).expect("ERROR: failed to read 'seq' line");
        plus.clear();
        my_buffer.read_line(&mut plus).expect("ERROR: failed to read 'plus' line");
        qual.clear();
        my_buffer.read_line(&mut qual).expect("ERROR: failed to read 'qual' line");
        id   = id.trim().to_string();
        seq  = seq.trim().to_string();
        plus = plus.trim().to_string();
        qual = qual.trim().to_string();

        // Test ID
        if id.chars().nth(0).unwrap() == '@' {
            id = format!("{} id-at:1", &id);
        } else {
            id = format!("{} id-at:0", &id);
        }

        // Test Seq
        id = format!("{} seq-length:{}", &id, seq.len());
        let mut illegal_seq_chars:String = String::new();
        if seq_regex.is_match(&seq) {
            for cap in seq_regex.captures_iter(&seq) {
                illegal_seq_chars.push_str(&cap[0]);
            }
        }
        id = format!("{} seq-invalid-chars:'{}'", &id, &illegal_seq_chars);

        // Test plus
        if plus.chars().nth(0).unwrap() == '+' {
            id = format!("{} id-plus:1", &id);
        } else {
            id = format!("{} id-plus:0", &id);
        }

        // Test qual
        let mut illegal_qual_chars:String = String::new();
        if qual_regex.is_match(&qual) {
            for cap in qual_regex.captures_iter(&qual) {
                illegal_qual_chars.push_str(&cap[0]);
            }
        }
        id = format!("{} qual-invalid-chars:'{}'", &id, &illegal_qual_chars);

        // quality score regex
        let mut qual_total :usize = 0;
        for q in qual.chars() {
            qual_total += q as usize;
        }
        let avg_qual :f32 = qual_total as f32 / qual.len() as f32 - 33.0;
        id = format!("{} avg-qual:{:.2}", &id, avg_qual);
        id = format!("{} qual-length:{}", &id, qual.len());

        let mut read_pair:u8 = (i % lines_per_read) + 1;
        if read_pair > 1 {
            read_pair = 2;
        }
        id = format!("{} read-pair:{}", &id, &read_pair);

        i += 4;

        println!("{}\n{}\n{}\n{}", id, seq, plus, qual);
    }
    /*
    if i % lines_per_read > 0{
        panic!("ERROR: incomplete fastq entry. Num lines: {}",i);
    }
    */

}


