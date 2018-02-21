extern crate ross;
extern crate getopts;
extern crate rand;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::env;
use getopts::Options;

use rand::thread_rng;
use rand::Rng;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    // ROSS flags.
    // TODO put these options into ROSS to streamline.
    opts.optflag("h", "help", "Print this help menu.");
    opts.optopt("n","numcpus","Number of CPUs (default: 1)","INT");

    opts.optopt("f","frequency","Frequency of sequences to print, 0 to 1. Default: 1","FLOAT");

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");

    if matches.opt_present("h") {
        println!("Ursula: downsample your reads\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let frequency :f32 = {
        if matches.opt_present("frequency") {
            matches.opt_str("frequency")
            .expect("ERROR parsing frequency parameter")
            .parse()
            .expect("ERROR parsing frequency as a float")
        } else {
            1.0
        }
    };

    let lines_per_read={
        if matches.opt_present("paired-end") {
            8
        }else{
            4
        }
    };

    let mut rng = thread_rng();

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut line_counter =0;
    let mut entry = String::new();
    for line in my_buffer.lines() {
        // unwrap the line here and shadow-set the variable.
        let line=line.expect("ERROR: did not get a line");
        line_counter+=1;
        entry.push_str(&line);

        // Action if we have a full entry when mod 0
        if line_counter % lines_per_read == 0 {
            // Should we print?
            if rng.gen_range(0.0,1.0) < frequency {
                println!("{}",entry);
            }
            // reset the entry string
            entry = String::new();
        }
    }
}

