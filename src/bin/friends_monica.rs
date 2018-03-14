extern crate getopts;
extern crate ross;
extern crate multiqueue;

use std::fs::File;
use std::io::BufReader;

use ross::ross_base_options;
use ross::io::fastq;
//use ross::io::seq::Seq;
use ross::io::seq::Cleanable;

use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = ross_base_options();

    // script-specific options
    opts.optopt("","min-length","Minimum length for each read in bp","INT");
    opts.optopt("","min-avg-quality","Minimum average quality for each read","FLOAT");
    opts.optopt("","min-trim-quality","Trim the edges of each read until a nucleotide of at least X quality is found","INT");

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");

    if matches.opt_present("h") {
        println!("{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let mut min_length :f32 = 0.0;
    if matches.opt_present("min-length") {
        min_length = matches.opt_str("min-length")
            .expect("ERROR: could not read the minimum length parameter")
            .parse()
            .expect("ERROR: min-length is not an integer");
    }

    let mut min_avg_qual :f32 = 0.0;
    if matches.opt_present("min-avg-quality") {
        min_avg_qual = matches.opt_str("min-avg-quality")
            .expect("ERROR: could not read the minimum average quality parameter")
            .parse()
            .expect("ERROR: min-avg-qual is not an integer");
    }

    let mut min_trim_qual :f32 = 0.0;
    if matches.opt_present("min-trim-quality") {
        min_trim_qual = matches.opt_str("min-trim-quality")
            .expect("ERROR: could not read the minimum average quality parameter")
            .parse()
            .expect("ERROR: min-trim-qual is not an integer");
    }

    // Read the file and send seqs to threads
    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let fastq_reader=fastq::FastqReader::new(my_buffer);
    for mut seq in fastq_reader {
        seq.thresholds.insert("min_length".to_string(),min_length);
        seq.thresholds.insert("min_avg_qual".to_string(),min_avg_qual);
        seq.thresholds.insert("min_trim_qual".to_string(),min_trim_qual);
        seq.lower_ambiguity_q();
        seq.trim();
        if seq.is_high_quality() {
           seq.print();
        }
    }
}

