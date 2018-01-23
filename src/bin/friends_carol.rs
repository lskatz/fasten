extern crate ross;
use std::fs::File;
use std::io::BufReader;

use ross::io::fastq;
use ross::io::seq::Cleanable;

#[test]
/// Test to see whether we read the challenge dataset correctly
fn challenge_dataset () {
    // Open the difficult file
    let challenge_file = File::open("testdata/four_reads.gt_16_lines.fastq").expect("Could not open testdata/four_reads.gt_16_lines.fastq");
    let challenge_buffer=BufReader::new(challenge_file);
    let mut challenge_reader=fastq::Reader::new(challenge_buffer);
    let mut challenge_string = String::new();
    while let Some(seq_obj) = challenge_reader.read_carefully() {
        challenge_string.push_str(&seq_obj.to_string());
    }

    // Open the easy file
    let easy_file  = File::open("testdata/four_reads.fastq").expect("Could not open testdata/four_reads.fastq");
    let easy_buffer= BufReader::new(easy_file);
    let mut easy_reader=fastq::Reader::new(easy_buffer);
    let mut easy_string = String::new();
    while let Some(seq_obj) = easy_reader.read_carefully() {
        easy_string.push_str(&seq_obj.to_string());
    }
    
    assert_eq!(challenge_string,easy_string);
}


fn main(){
    
    //ross::parse_args();

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut fastq_reader=fastq::Reader::new(my_buffer);
    while let Some(seq_obj) = fastq_reader.read_carefully() {
        seq_obj.print();
    }
}

