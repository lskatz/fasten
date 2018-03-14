extern crate getopts;
extern crate ross;
extern crate regex;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use regex::Regex;

use ross::ross_base_options;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = ross_base_options();
    // Options specific to this script
    opts.optopt("","min-length","Minimum read length allowed","INT");
    opts.optopt("","min-quality","Minimum quality allowed","FLOAT");
    opts.optflag("","paired-end","The reads are interleaved paired-end");
    opts.optflag("","print","Print the reads as they are being validated (useful for unix pipes)");
    opts.optflag("v","verbose","");

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");
    if matches.opt_present("help") {
        println!("Validates your reads and makes you feel good about yourself!\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);

    let mut in_middle_of_entry=false;
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
    let should_print=matches.opt_present("print");

    // If there is a match on these, then mark invalid.
    // In other words, we are looking for a pattern that
    // is NOT the target seq or qual
    let seq_regex = Regex::new(r"[^a-zA-Z]").expect("malformed seq regex");
    let qual_regex= Regex::new(r"[^!-Z]").expect("malformed qual regex");

    // TODO have a print buffer, something like 4096 bytes

    for (i,line) in my_buffer.lines().enumerate() {
        let line=line.expect("ERROR: did not get a line");
        if should_print {
            println!("{}",line);
        }

        // TODO pattern match for each kind of line:
        // id, seq, +, qual
        match i%lines_per_read {
            0=>{
                in_middle_of_entry=true;
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
                in_middle_of_entry=false;
                if qual_regex.is_match(&line) {
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
    }
    if in_middle_of_entry {
        panic!("ERROR: incomplete fastq entry");
    }

    if matches.opt_present("verbose") {
        ross::logmsg("These reads have been validated!");
    }
}


