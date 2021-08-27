extern crate getopts;
extern crate fasten;
extern crate regex;
extern crate rand;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use rand::distributions::{IndependentSample, Range};

use fasten::fasten_base_options;
use fasten::logmsg;


fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = fasten_base_options();
    // Options specific to this script
    opts.optopt("s", "snps", "Number of SNPs (point mutations) to include per read.", "INT");
    opts.optflag("m", "mark", "lowercase all reads but uppercase the SNPs (not yet implemented)");

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");
    if matches.opt_present("help") {
        println!("Mutates reads. There is no mutation model; only randomness.\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    if matches.opt_present("paired-end") {
        logmsg("WARNING: --paired-end is not utilized in this script");
    }

    // Figure out the number of SNPs per read
    let num_snps:u8 = if matches.opt_present("snps") { 
        matches.opt_str("snps").unwrap()
            .parse().expect("--snps needs to be a FLOAT")
    } else {
        panic!("ERROR: --snps is required")
    };

    // Not sure if I should expose NTs to a flag if someone
    // wants more nucleotide codes like N.
    let mark:bool = if matches.opt_present("mark"){
        true
    } else {
        false
    };


    let nts = vec!['A', 'C', 'G', 'T'];
    //let nts = vec!['a', 'c', 'g', 't'];

    // Make this one time outside the loop to keep overhead low
    //let mut rng = rand::thread_rng();

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut buffer_iter = my_buffer.lines();
    while let Some(line) = buffer_iter.next() {
        let id  = line.expect("ERROR reading the ID line");
        let seq = buffer_iter.next().expect("ERROR reading a sequence line")
            .expect("ERROR reading a sequence line");
        buffer_iter.next().expect("ERROR reading a plus line")
            .expect("ERROR reading the plus line");
        let qual= buffer_iter.next().expect("ERROR reading a qual line")
            .expect("ERROR reading a qual line");

        let new_seq = mutate(&seq, &nts, num_snps, mark);

        println!("{}\n{}\n+\n{}",id,new_seq,qual);

    }
}

fn mutate(seq: &str, nts: &Vec<char>, num_snps: u8, mark:bool) -> String {
    let mut sequence:Vec<u8> = seq.as_bytes().to_vec();
    if mark {
        sequence.make_ascii_lowercase();
    }
    let length = sequence.len();
    let between = Range::new(0, length);
    let nt_range= Range::new(0, nts.len());
    let mut rng = rand::thread_rng();
    for _ in 0..num_snps {
        let pos = between.ind_sample(&mut rng);
        let nt  = nts[nt_range.ind_sample(&mut rng)];
        sequence[pos] = nt as u8;
    }
    return String::from_utf8_lossy(&sequence).to_string();
}

