extern crate getopts;
extern crate ross;
extern crate rand;
use std::fs::File;
use std::io::BufReader;
use std::env;
use std::collections::HashMap;

use rand::{Rng,thread_rng};

use ross::ross_base_options;
use ross::io::fastq;
use ross::io::seq::Seq;
use ross::io::seq::Cleanable;


fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = ross_base_options();
    //script-specific flags
    opts.optopt("r","readlength","Read length (default: 150)","INT");
    //opts.optopt("n","numbases","Maximum number of nucleotides (default: 0, unlimited)","INT");
    //opts.optopt("n","numreads","Maximum number of reads (default: 0, unlimited)","INT");
    opts.optflag("p","paired-end","Paired end reads");

    let matches = opts.parse(&args[1..]).expect("Error: could not parse parameters");
    if matches.opt_present("help") {
        println!("Create random reads from stdin. Phoebe is totally random!\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let is_pe = matches.opt_present("paired-end");
    println!("{}",is_pe);

    print_reads_from_stdin(is_pe);
}

fn print_reads_from_stdin(is_pe: bool) -> () {
    // Start off with a capacity of 10k reads.
    let mut seqs :Vec<Seq> = Vec::with_capacity(10000);
    let mut seqs_pe :HashMap<String,Seq> = HashMap::with_capacity(10000);
    let my_file = File::open("/dev/stdin").expect("Could not open stdin");
    let my_buffer=BufReader::new(my_file);
    let mut fastq_reader=fastq::FastqReader::new(my_buffer);
    while let Some(mut seq) = fastq_reader.next() {
        if is_pe {
            let mut seq2 = fastq_reader.next().expect("ERROR: could not read the entry pair");
            seq2.pairid=seq.id.clone();
            seq.pairid=seq2.id.clone();

            seqs_pe.insert(seq2.id.clone(), seq2);
        }
        seqs.push(seq);
    }

    // choose random reads
    let mut rng = thread_rng();
    rng.shuffle(&mut seqs);
    for seq in seqs {
        seq.print();
        if is_pe {
            seqs_pe.entry(seq.pairid.trim().to_string())
                .or_insert(Seq::blank())
                .print();
        }
    }
}


