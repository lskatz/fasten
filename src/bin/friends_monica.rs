extern crate ross;
use std::fs::File;
use std::io::BufReader;

use std::sync::mpsc;
use std::thread;

use ross::io::fastq;
use ross::io::seq::Cleanable;

fn main(){
    ross::parse_args();

    // receiving threads
    let numcpus=1;
    let (tx,rx)=mpsc::channel();
    
    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut fastq_reader=fastq::Reader::new(my_buffer);
    // TODO send a bunch of seq_obj at once (buffer)
    // TODO spawn threads
    while let Some(seq_obj) = fastq_reader.read_carefully() {
        tx.send(seq_obj)
            .expect("Could not send a seq object down the channel");
    }
    for mut seq_obj in rx {
        seq_obj.lower_ambiguity_q();
        seq_obj.trim();
        if seq_obj.is_high_quality() {
            seq_obj.print();
        }
    }
}

