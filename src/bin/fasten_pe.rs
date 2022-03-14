//! Determine paired-end-ness in an interleaved file.
//! Exit code of 0 indicates PE. Exit code > 0 indicates single end.
//! 
//! # Examples
//! 
//! ```bash
//! # Test the file and then print a message with the exit code
//! cat file.fastq | fasten_pe; echo "Reads were paired-end? $?";
//! # Test the file and if it is paired end (exit code 0), then print a message
//! cat file.fastq | fasten_pe || echo "Reads were paired end.";
//! ```
//! 
//! ```text
//! # Usage
//! 
//! Usage: fasten_pe [-h] [-n INT] [-p] [-v] [--print-reads]
//! 
//! Options:
//!     -h, --help          Print this help menu.
//!     -n, --numcpus INT   Number of CPUs (default: 1)
//!     -p, --paired-end    The input reads are interleaved paired-end
//!     -v, --verbose       Print more status messages
//!         --print-reads   Print each read. Useful for Unix pipes.
//! ```    
extern crate fasten;
extern crate regex;

use fasten::fasten_base_options;
use fasten::logmsg;
use regex::Regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = fasten_base_options();
    //opts.optopt("c","check-first","How many deflines to check to make sure the input is paired-end","INT");
    opts.optflag("","print-reads","Print each read. Useful for Unix pipes.");
    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");
    
    if matches.opt_present("help") {
        println!("Determine paired-end-ness in an interleaved file. Exit code of 0 indicates PE. Exit code > 0 indicates SE.\n{}", 
                 opts.usage(&opts.short_usage(&args[0]))
                );
        std::process::exit(0);
    }
    if matches.opt_present("paired-end") {
        logmsg("WARNING: --paired-end was supplied but this script is supposed to determine whether the input is paired-end.");
    }

    let should_print = matches.opt_present("print-reads");

    // Save the top X ID pairs in a vector
    // and compare them in several functions that test for
    // IDs. If any test returns true, then we can say that
    // it is paired-end input.
    let mut id1_vec :Vec<String>=Vec::new();
    let mut id2_vec :Vec<String>=Vec::new();
    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut lines = my_buffer.lines();
    while let Some(line) = lines.next() {
        let id1 = line.expect("ERROR parsing id line in R1");
        let mut entry=id1.clone();
        entry.push('\n');
        for _ in 1..4 { // move ahead three lines
            let other_line = lines.next()
                .expect("ERROR getting next line")
                .expect("ERROR parsing next line");
            entry.push_str(&other_line);
            entry.push('\n');
        }
        let id2 = lines.next()
            .expect("ERROR getting R2. This is not a paired end file.")
            .expect("ERROR parsing next line in R2");
        entry.push_str(&id2);
        entry.push('\n');
        for _ in 1..4 { // move ahead three lines
            let other_line = lines.next()
                .expect("ERROR getting next line in R2. This is not a paired end file.")
                .expect("ERROR parsing next line in R2");
            entry.push_str(&other_line);
            entry.push('\n');
        }
        id1_vec.push(id1);
        id2_vec.push(id2);

        if should_print {
            print!("{}",entry);
        }
    }

    // For every function we can think of, throw the ID
    // vectors against them. If any of them return >0,
    // then we have a positive
    let mut is_paired_end :u8 = 0;
    is_paired_end += is_paired_end_slash12(&id1_vec, &id2_vec);
    is_paired_end += is_paired_end_miseq(&id1_vec, &id2_vec);

    // If this is not paired end, return a nonzero
    if is_paired_end == 0 {
        if matches.opt_present("verbose") {
            logmsg("I do not think this is interleaved paired end");
        }
        std::process::exit(1);
    }
    
    if matches.opt_present("verbose") {
        logmsg("The fastq input seems to be interleaved paired-end");
    }

}

fn is_paired_end_slash12(id1_ref: &Vec<String>, id2_ref: &Vec<String>) -> u8 {

    // We are using iterators and want to reset them
    let id1_vec = id1_ref.clone();
    let id2_vec = id2_ref.clone();
    
    // @.../1
    let slash_r1r2_regex = Regex::new(r"(.+)/([12])$").expect("malformed qual regex");

    let mut id2_iter = id2_vec.iter();
    for id1 in id1_vec.iter(){
        let id2 = id2_iter.next()
            .expect("ERROR getting next id2");

        let caps1_result = slash_r1r2_regex.captures(&id1);
        let caps2_result = slash_r1r2_regex.captures(&id2);

        // Test for whether the regex matched
        if caps1_result.is_none() || caps2_result.is_none() {
            return 0;
        }

        // If we have a match then get the captured values
        let caps1 = caps1_result
            .expect("ERROR: could not regex against id1");
        let caps2 = caps2_result
            .expect("ERROR: could not regex against id2");

        // Make sure the base name matches
        if caps1[1] != caps2[1] {
            return 0;
        }
        // Make sure there is a 1/2 combo
        if &caps1[2] != "1" || &caps2[2] != "2" {
            return 0;
        }
    }

    return 1;
}

 // http://support.illumina.com/content/dam/illumina-support/help/BaseSpaceHelp_v2/Content/Vault/Informatics/Sequencing_Analysis/BS/swSEQ_mBS_FASTQFiles.htm
fn is_paired_end_miseq(id1_ref: &Vec<String>, id2_ref: &Vec<String>) -> u8 {
    // We are using iterators and want to reset them
    let id1_vec = id1_ref.clone();
    let id2_vec = id2_ref.clone();
    let mut id2_iter = id2_vec.iter();

    for id1 in id1_vec.iter(){
        let id2 = id2_iter.next()
            .expect("ERROR getting next id2");


        // Get the 7th field which is a combination of the
        // Y position and the read number, separated by a
        // space.
        let id1_tmp = id1.split(":").nth(6);
        let id2_tmp = id2.split(":").nth(6);
        if id1_tmp.is_none() || id2_tmp.is_none() {
            return 0;
        }

        // Get the read number. It has to be 1 and 2.
        let id1_read_number = id1_tmp.unwrap().split(" ").nth(1);
        let id2_read_number = id2_tmp.unwrap().split(" ").nth(1);
        if id1_read_number.is_none() || id2_read_number.is_none() {
            return 0;
        }
        if id1_read_number.unwrap() != "1" || id2_read_number.unwrap() != "2" {
            return 0;
        }
    }

    return 1;
}

