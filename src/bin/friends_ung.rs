extern crate ross;

use ross::ross_base_options;
use ross::io::fastq;

use std::fs::File;
use std::io::BufReader;
use std::env::args;
//use std::io;

fn main() {
    let mut opts = ross_base_options();
    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");
    if matches.opt_present("help") {
        println!("Convert a fastq file to a standard 4-lines-per-entry format\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let fastq_reader=fastq::FastqReader::new_careful(my_buffer);
    for Some(seq1) in fastq_reader.iter() {
        let seq2 = fastq_reader.next();

        println!("{}\n{}",seq1.id,seq2.id);
        break;

    }
}
