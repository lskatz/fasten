extern crate ross;
use std::fs::File;
use std::io::BufReader;
//use std::io::prelude::*;

use ross::io::fastq;
use ross::io::seq::Cleanable;

fn main(){
    
    //ross::parse_args();

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut fastq_reader=fastq::Reader::new(my_buffer);
    while let Some(seq_obj) = fastq_reader.read_carefully() {
        seq_obj.print();
    }
}

