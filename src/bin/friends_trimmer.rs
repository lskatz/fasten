extern crate ross;
extern crate statistical;
extern crate getopts;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use ross::ross_base_options;
use ross::logmsg;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = ross_base_options();

    // script-specific options
    opts.optopt("f","first-base","The first base to keep (default: 0)","INT");
    opts.optopt("l","last-base","The last base to keep. If negative, counts from the right. (default: 0)","INT");

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");

    if matches.opt_present("help") {
        println!("Blunt-end trims using 0-based coordinates\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }
    if matches.opt_present("paired-end") {
        logmsg("WARNING: --paired-end is not utilized in this script");
    }

    let first_base ={
        if matches.opt_present("first-base") {
            matches.opt_str("first-base")
                .expect("ERROR: could not understand parameter --first-base")
                .parse()
                .expect("ERROR: --first-base is not an INT")
        } else {
            0
        }
    };

    let mut last_base ={
        if matches.opt_present("last-base") {
            matches.opt_str("last-base")
                .expect("ERROR: could not understand parameter --last-base")
                .parse()
                .expect("ERROR: --last-base is not an INT")
        } else {
            0
        }
    };


    let filename = "/dev/stdin";
    
    // read the file
    let my_file = File::open(&filename).expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut num_lines=0;
    for line in my_buffer.lines() {
        num_lines+=1;

        // Every other line gets trimmed
        match num_lines % 2 {
            0 => {
                let seq_or_qual = line.expect("ERROR reading line");
                let length = seq_or_qual.len();
                if last_base == 0 || last_base > length {
                    last_base = length;
                }
                println!("{}",&seq_or_qual[first_base..last_base]);
            }
            _ => {
                println!("{}",&line.expect("ERROR reading id or plus line")); 
            }
        };
    }

}

