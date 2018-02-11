extern crate getopts;
extern crate ross;
extern crate rand;
use std::fs::File;
use std::io::BufReader;
use std::env;
use getopts::Options;
use rand::{Rng,thread_rng};

use ross::io::fastq;
use ross::io::seq::Seq;
use ross::io::seq::Cleanable;


fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    //ROSS flags
    opts.optflag("h","help","Print this help menu.");
    opts.optopt("n","numcpus","Number of CPUs (default: 1)", "INT");
    
    //script-specific flags
    opts.optopt("r","readlength","Read length (default: 150)","INT");
    opts.optopt("n","numbases","Maximum number of nucleotides (default: 0, unlimited)","INT");
    opts.optopt("n","numreads","Maximum number of reads (default: 0, unlimited)","INT");
    opts.optflag("p","paired-end","Paired end reads");

    let matches = opts.parse(&args[1..]).expect("Error: could not parse parameters");
    if matches.opt_present("h") {
        println!("Create random reads, or choose random reads from an input file. Phoebe is totally random!\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    if args.len() > 0 {
        print_reads_from_files(&Vec::from(&args[1..]));
    } else {
        print_random_reads();
    }
}

fn print_reads_from_files(file: &Vec<String>) -> () {
    // collect all reads from all files
    let mut seqs :Vec<Seq> = vec![];
    for filename in file.iter() {
        let my_file = File::open(&filename).expect("Could not open file");
        let my_buffer=BufReader::new(my_file);
        let fastq_reader=fastq::FastqReader::new(my_buffer);
        for seq in fastq_reader {
            // TODO add 'pair' trait to seq struct,
            // so that we can have PE reads easily.
            seqs.push(seq);
        }
    }

    // choose random reads
    let mut rng = thread_rng();
    rng.shuffle(&mut seqs);
    for seq in seqs {
        seq.print();
    }
}

fn print_random_reads() -> () {

}

