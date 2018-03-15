extern crate ross;
extern crate regex;

use ross::ross_base_options;
use regex::Regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = ross_base_options();
    opts.optflag("x","blah","blah!");
    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");
    
    // If there is a match on these, then mark invalid.
    // In other words, we are looking for a pattern that
    // is NOT the target seq or qual
    let seq_regex = Regex::new(r"[^a-zA-Z]").expect("malformed seq regex");
    let qual_regex= Regex::new(r"[^!-Z]").expect("malformed qual regex");

    if matches.opt_present("help") {
        println!("Convert a fastq file to a standard 4-lines-per-entry format\n{}", 
                 opts.usage(&opts.short_usage(&args[0]))
                );
        std::process::exit(0);
    }

    let lines_per_read={
        if matches.opt_present("paired-end") {
            8
        }else{
            4
        }
    };

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    for (i,line) in my_buffer.lines().enumerate() {
        let line = line.expect("ERROR: could not read the next line in the input");
        //match i%lines_per_read {
    }
}

