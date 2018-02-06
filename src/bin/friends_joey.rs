extern crate getopts;
extern crate ross;
use std::fs::File;
use std::io::BufReader;

use ross::io::fastq;
use ross::io::seq::Cleanable;
use ross::io::seq::Seq;

use std::env;
use getopts::Options;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    //ROSS flags
    opts.optflag("h","help","Print this help menu.");
    opts.optopt("n","numcpus","Number of CPUs (default: 1)", "INT");
    let matches = opts.parse(&args[1..]).expect("Error: could not parse parameters");
    if matches.opt_present("h") {
        println!("Interleaves reads.\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    // Read stdin: read 1 first, and read 2 is halfway down.
    // Unfortunately this means that it all goes into ram.
    let my_file = File::open("/dev/stdin").expect("Could not open stdin");
    let my_buffer=BufReader::new(my_file);
    let fastq_reader=fastq::FastqReader::new(my_buffer);
    let seqs :Vec<Seq> = fastq_reader.collect();
    let seq_offset = seqs.len() / 2;

    // Interleaving magic
    for i in 0..seq_offset {
        seqs[i].print();
        seqs[i+seq_offset].print();
    }
}


