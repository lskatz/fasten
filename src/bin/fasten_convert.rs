extern crate getopts;
extern crate fasten;
extern crate fastq;
//use std::fs::File;
//use std::io::BufReader;

use fasten::fasten_base_options;
//use fasten::io::fastq;
//use fasten::io::seq::Cleanable;
//use fasten::logmsg;

use std::io::stdin;
use fastq::{Parser, Record};

use std::env;

use std::sync::mpsc::channel;

/// Struct that can handle paired end reads
#[derive(Debug, Clone)]
struct FastenSeq {
  id1:   String,
  seq1:  String,
  qual1: String,
  id2:   String,
  seq2:  String,
  qual2: String,
}
impl FastenSeq{
  fn new() -> FastenSeq{
    return FastenSeq{
      id1:   String::new(),
      seq1:  String::new(),
      qual1: String::new(),
      id2:   String::new(),
      seq2:  String::new(),
      qual2: String::new(),
    };
  }
}
      
/*
impl Copy for FastenSeq {
  fn Copy(&self) -> FastenSeq {
    return FastenSeq{
      id1:   self.id1.clone(),
      seq1:  self.seq1.clone(),
      qual1: self.qual1.clone(),
      id2:   self.id1.clone(),
      seq2:  self.seq1.clone(),
      qual2: self.qual1.clone(),
    }
  }
}
*/

#[test]
/// Test to see whether we read the challenge dataset correctly
fn challenge_dataset () {
    // Open the difficult file
    let challenge_file = File::open("testdata/four_reads.gt_16_lines.fastq").expect("Could not open testdata/four_reads.gt_16_lines.fastq");
    let challenge_buffer=BufReader::new(challenge_file);
    let challenge_reader=fastq::FastqReader::new_careful(challenge_buffer);
    let mut challenge_string = String::new();
    for seq_obj in challenge_reader {
        challenge_string.push_str(&seq_obj.to_string());
    }

    // Open the easy file
    let easy_file  = File::open("testdata/four_reads.fastq").expect("Could not open testdata/four_reads.fastq");
    let easy_buffer= BufReader::new(easy_file);
    let easy_reader=fastq::FastqReader::new(easy_buffer);
    let mut easy_string = String::new();
    for seq_obj in easy_reader {
        easy_string.push_str(&seq_obj.to_string());
    }
    
    assert_eq!(challenge_string,easy_string);
}


fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = fasten_base_options();
    opts.optopt("i", "in-format",  "The input format for stdin",  "STR");
    opts.optopt("o", "out-format", "The output format for stdin", "STR");

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");
    if matches.opt_present("help") {
        println!("Convert a fastq file to a standard 4-lines-per-entry format\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let in_format  = matches.opt_default("in-format", "fastq")
                       .unwrap_or(String::from("fastq"))
                       .to_lowercase();
    let out_format = matches.opt_default("out-format","fastq")
                       .unwrap_or(String::from("fastq"))
                       .to_lowercase();

    let (tx, rx):(std::sync::mpsc::Sender<FastenSeq>,std::sync::mpsc::Receiver<FastenSeq>) = channel();

    //TODO (?) multithread this 
    match in_format.as_str() {
      "fastq" => {read_fastq(tx, &matches);}
      _ => {panic!("Unknown input format {}", in_format);}
    };

    match out_format.as_str() {
      "fastq" => {write_fastq(rx);}
      _ => {panic!("Unknown output format {}", out_format);}
    };

}

fn read_fastq(tx:std::sync::mpsc::Sender<FastenSeq>, matches:&getopts::Matches){
  let paired_end = matches.opt_present("paired-end");

  let parser = Parser::new(stdin());

  let mut parser_getter = parser.ref_iter();
  parser_getter.advance().expect("Could not read the first fastq entry");
  while let Some(record1) = parser_getter.get() {
    let mut seq:FastenSeq = FastenSeq::new();
    seq.id1   = std::str::from_utf8(record1.head()).unwrap().to_string();
    seq.seq1  = std::str::from_utf8(record1.seq()).unwrap().to_string();
    seq.qual1 = std::str::from_utf8(record1.qual()).unwrap().to_string();
    if paired_end {
      // get the next entry with advance() and then get()
      match &parser_getter.advance() {
        Ok(_) => {},
        Err(err) => {
          panic!("ERROR: could not read the second entry in a paired end read: {}", err);
        }
      };
      let record2 = &parser_getter.get().expect("ERROR parsing second pair in a paired end read");
      seq.id2   = std::str::from_utf8(record2.head()).unwrap().to_string();
      seq.seq2  = std::str::from_utf8(record2.seq()).unwrap().to_string();
      seq.qual2 = std::str::from_utf8(record2.qual()).unwrap().to_string();
    }

    tx.send(seq).expect("Sending seq object to writer");

    match &parser_getter.advance() {
      Ok(_) => {},
      Err(_) => {break;}
    };
  }
}

fn write_fastq(rx:std::sync::mpsc::Receiver<FastenSeq>){
  //while let Some(seq) = rx.recv(){

  //}
}
