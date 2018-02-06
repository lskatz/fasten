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
    
    //script-specific flags
    opts.optflag("d","deshuffle","Deshuffle reads from stdin");
    opts.optopt("1","","Forward reads. If deshuffling, reads are written to this file.","1.fastq");
    opts.optopt("2","","Forward reads. If deshuffling, reads are written to this file.","2.fastq");

    let matches = opts.parse(&args[1..]).expect("Error: could not parse parameters");
    if matches.opt_present("h") {
        println!("Interleaves reads from either stdin or file parameters.\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    if matches.opt_present("d") {
        deshuffle(&matches);
    } else {
        shuffle(&matches);
    }
}

fn deshuffle(matches: &getopts::Matches) -> () {
    
    // Where are we reading to?  Get those filenames.
    let r1_filename = if matches.opt_present("1") {
        matches.opt_str("1").unwrap()
    } else {
        "/dev/stdout".to_string()
    };
    let r2_filename = if matches.opt_present("2") {
        matches.opt_str("2").unwrap()
    } else {
        "/dev/stdout".to_string()
    };

    // read stdin
    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let fastq_reader=fastq::FastqReader::new(my_buffer);
    for seq in fastq_reader {
        // print to file 1 and to file 2, alternating each Seq
    }

    panic!("not implemented");
}

fn shuffle(matches: &getopts::Matches) -> () {

    // Where are we reading from?  Get those filenames.
    let r1_filename = if matches.opt_present("1") {
        matches.opt_str("1").unwrap()
    } else {
        "/dev/stdin".to_string()
    };
    let r2_filename = if matches.opt_present("2") {
        matches.opt_str("2").unwrap()
    } else {
        "/dev/stdin".to_string()
    };

    // Read 1 first, and read 2 is halfway down.
    // Unfortunately this means that it all goes into ram.
    let     seqs1 = read_seqs(&r1_filename);
    let mut seqs2 = read_seqs(&r2_filename);
    let mut num_pairs = seqs1.len();

    // If reading R1 from stdin, it is possible that seqs2 
    // is empty. If so, redistribute half the reads from 
    // seqs1 into seqs2.
    if seqs2.len() == 0 {
        num_pairs = seqs1.len()/2;
        for seq in seqs1[seqs1.len()/2..seqs1.len()].iter() {
            seqs2.push(seq.clone());
        }
    }

    for i in  0..num_pairs  {
        seqs1[i].print();
        seqs2[i].print();
    }

}

fn read_seqs(filename: &String) -> Vec<Seq> {

    let my_file = File::open(&filename).expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let fastq_reader=fastq::FastqReader::new(my_buffer);
    let seqs :Vec<Seq> = fastq_reader.collect();
    return seqs;
}
