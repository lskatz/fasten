extern crate ross;
extern crate regex;

use ross::ross_base_options;
use ross::logmsg;
use regex::Regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = ross_base_options();
    opts.optopt("c","check-first","How many deflines to check to make sure the input is paired-end","INT");
    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");
    
    let slash_r1r2_regex = Regex::new(r"(.+)/([12])$").expect("malformed qual regex");

    if matches.opt_present("help") {
        println!("Determine paired-end-ness in an interleaved file. Currently only checks deflines for the /1 and /2 format\n{}", 
                 opts.usage(&opts.short_usage(&args[0]))
                );
        std::process::exit(0);
    }

    let check_first = { 
        if matches.opt_present("check-first") {
            matches.opt_str("check-first")
                .expect("Error reading the check-first option")
                .parse()
                .expect("ERROR converting the check-first parameter to an integer")
        } else {
            200
        }
    };

    let mut id1=String::new();
    let mut id2 :String;
    let mut pairs_counter=0;

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    for (i,line) in my_buffer.lines().enumerate() {
        let line = line.expect("ERROR: could not read the next line in the input");
        match i%8 {
            0=>{
                id1=line;
            }
            4=>{
                id2=line;
                pairs_counter+=1;

                let caps1 = slash_r1r2_regex.captures(&id1).expect("ERROR: could not regex against id1");
                let caps2 = slash_r1r2_regex.captures(&id2).expect("ERROR: could not regex against id2");

                // Make sure the base name matches
                if caps1[1] != caps2[1] {
                    let mut msg = "ID1 does not match ID2 on line ".to_string();
                    msg.push_str(&i.to_string());
                    msg.push_str("\n");
                    msg.push_str(&id1);
                    msg.push_str(" vs ");
                    msg.push_str(&id2);
                    logmsg(&msg);
                    std::process::exit(1);
                }
                // Make sure there is a 1/2 combo
                if &caps1[2] != "1" || &caps2[2] != "2" {
                    let mut msg = "/1 is not followed by /2 on line ".to_string();
                    msg.push_str(&i.to_string());
                    msg.push_str("\n");
                    msg.push_str(&id1);
                    msg.push_str(" vs ");
                    msg.push_str(&id2);
                    logmsg(&msg);
                    std::process::exit(1);
                }

                if pairs_counter >= check_first {
                    break;
                }

            }
            // We can safely ignore the seq, plus, and qual lines
            _=>{ }
        }
    }

    if matches.opt_present("verbose") {
        logmsg("The fastq input seems to be interleaved paired-end");
    }
}

