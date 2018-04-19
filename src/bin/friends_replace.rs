extern crate getopts;
extern crate ross;
extern crate regex;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use regex::Regex;

use ross::ross_base_options;
use ross::logmsg;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = ross_base_options();
    // Options specific to this script
    opts.optopt("f","find","Regular expression (default: '.')","STRING");
    opts.optopt("r","replace","String to replace each match","STRING");
    opts.optopt("w","which","Which field to match on? ID, SEQ, QUAL. Default: SEQ","STRING");

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");
    if matches.opt_present("help") {
        println!("Streaming editor for fastq data using a find/replace.\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    if matches.opt_present("paired-end") {
        logmsg("WARNING: --paired-end is not utilized in this script");
    }

    //which field does the user want to find and replace?
    let which_field={
        if matches.opt_present("which") {
            matches.opt_str("which").expect("ERROR parsing --which")
                .to_uppercase()
        } else {
            "SEQ".to_string()
        }
    };

    // Figure out what we are searching for
    let find_param={
        if matches.opt_present("find") {
            matches.opt_str("find")
                .expect("ERROR: could not parse find parameter")
        } else {
            String::from(".")
        }
    };

    // form the regex
    let find_regex = Regex::new(&find_param)
        .expect("malformed seq regex given by --find");

    // Make the replace string
    let replace :String = {
        if matches.opt_present("replace") {
            matches.opt_str("replace")
                .expect("ERROR: could not parse --replace parameter")
        } else {
            String::from("")
        }
    };
    // but it requires a &str
    let replace_str :&str = replace.as_str();

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut buffer_iter = my_buffer.lines();
    while let Some(line) = buffer_iter.next() {
        let mut id  = line.expect("ERROR reading the ID line");
        let mut seq = buffer_iter.next().expect("ERROR reading a sequence line")
            .expect("ERROR reading a sequence line");
                  buffer_iter.next().expect("ERROR reading a plus line")
                      .expect("ERROR reading the plus line");
        let mut qual= buffer_iter.next().expect("ERROR reading a qual line")
            .expect("ERROR reading a qual line");

        // Find and replace
        if &which_field == "SEQ" {
            seq  = find_regex.replace_all(&seq, replace_str).into_owned();
        } else if &which_field == "QUAL" {
            qual = find_regex.replace_all(&qual, replace_str).into_owned();
        } else if &which_field == "ID" {
            id   = find_regex.replace_all(&id, replace_str).into_owned();
        } else {
            panic!("Not implemented for --which: {}", which_field);
        }
        println!("{}\n{}\n+\n{}",id,seq,qual);

    }
}

