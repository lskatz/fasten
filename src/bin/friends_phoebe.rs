extern crate getopts;
extern crate ross;
extern crate rand;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use rand::{Rng,thread_rng};

use ross::ross_base_options;

fn main(){
    let args: Vec<String> = env::args().collect();
    let opts = ross_base_options();
    //script-specific flags

    let matches = opts.parse(&args[1..]).expect("Error: could not parse parameters");
    if matches.opt_present("help") {
        println!("Create random reads from stdin. Phoebe is totally random!\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let lines_per_read :u32={
        if matches.opt_present("paired-end") {
            8
        }else{
            4
        }
    };

    print_reads_from_stdin(lines_per_read);
}

fn print_reads_from_stdin(lines_per_read :u32) -> () {
    // Start off with a capacity of 100k reads.
    let mut seqs :Vec<String> = Vec::with_capacity(100000);
    let my_file = File::open("/dev/stdin").expect("Could not open stdin");
    let my_buffer=BufReader::new(my_file);
    let mut lines = my_buffer.lines();
    while let Some(id) = lines.next() {
        let mut entry = id.expect("ERROR: could not parse the ID line");
        for _ in 1..lines_per_read {
            entry.push('\n');
            let next_line = lines.next()
                .expect("ERROR: could not get the next line")
                .expect("ERROR: could not parse the next line");
            entry.push_str(&next_line);
        }

        seqs.push(entry);
    }

    // choose random reads
    let mut rng = thread_rng();
    rng.shuffle(&mut seqs);
    for seq in seqs {
        println!("{}",seq);
    }
}


