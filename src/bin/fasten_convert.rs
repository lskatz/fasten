extern crate getopts;
extern crate fasten;
extern crate fastq;
extern crate bam;
extern crate bio;

use bam::RecordReader;
use bio::io::fastq::FastqRead;    
use bio::io::fasta::FastaRead;    

use fasten::fasten_base_options;
use fasten::logmsg;

use std::env;
//use std::fmt;

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
  /// a blank new object is a set of blank strings for each value
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
  fn as_fastq(&self) -> String {
    let qual1:String = {
      if self.qual1.is_empty() {
        String::from_utf8(vec![b'I'; self.seq1.len()]).unwrap()
      } else {
        String::from(&self.qual1)
      }
    };
    let mut entry:String = format!("@{}\n{}\n+\n{}",
                             self.id1, self.seq1, &qual1);
    if !self.id2.is_empty() {
      let qual2:String = {
        if self.qual2.is_empty() {
          String::from_utf8(vec![b'I'; self.seq2.len()]).unwrap()
        } else {
          String::from(&self.qual2)
        }
      };
      entry = format!("{}\n@{}\n{}\n+\n{}",
                entry, self.id2, self.seq2, &qual2);
    }
    return entry;
  }

  fn as_fasta(&self) -> String {
    let mut entry:String = format!(">{}\n{}",
                             self.id1, self.seq1);
    if !self.id2.is_empty() {
      entry = format!("{}\n>{}\n{}",
                entry, self.id2, self.seq2);
    }
    return entry;
  }

  fn as_sam(&self) -> String {
    let mut flag = "4"; // unmapped

    if !self.id2.is_empty() {
      flag = "77"; // unmapped + pair unmapped + has pair + first in pair
    }

    let mut entry:String = vec![self.id1.as_str(), flag, "*", "0", "0", "*", "*", "0", "0", self.seq1.as_str(), self.qual1.as_str()].join("\t");
    if !self.id2.is_empty() {
      let entry2:String  = vec![self.id2.as_str(), "141", "*", "0", "0", "*", "*", "0", "0", self.seq2.as_str(), self.qual2.as_str()].join("\t");
      entry = format!("{}\n{}", entry, entry2);
    }

    return entry;
  }
}
      
fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = fasten_base_options();
    opts.optopt("i", "in-format",  "The input format for stdin",  "STR");
    opts.optopt("o", "out-format", "The output format for stdin", "STR");

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");
    if matches.opt_present("help") {
        println!("Convert from a format to another format. Possible choices are: fastq, fasta, sam.{}", opts.usage(&opts.short_usage(&args[0])));
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
      "sam"   => {read_sam(tx, &matches);}
      "fasta" => {read_fasta(tx, &matches);}
      _ => {panic!("Unknown input format {}", in_format);}
    };

    match out_format.as_str() {
      "fastq" => {write_fastq(rx);}
      "fasta" => {write_fasta(rx);}
      "sam"   => {write_sam(rx);}
      _ => {panic!("Unknown output format {}", out_format);}
    };

}

fn read_fasta(tx:std::sync::mpsc::Sender<FastenSeq>, matches:&getopts::Matches){
  if matches.opt_present("paired-end") {
    logmsg("Warning: paired end is not implemented for input fasta");
  }

  let mut reader = bio::io::fasta::Reader::new(std::io::stdin());
  let mut record = bio::io::fasta::Record::new();

  loop{
    match reader.read(&mut record) {
      Ok(()) => {
        if record.is_empty() {
          break;
        }
      },
      Err(e)   => {panic!("{}",e);},
    };

    let mut seq:FastenSeq = FastenSeq::new();
    seq.id1   = record.id().to_string();
    seq.seq1  = std::str::from_utf8(record.seq()).unwrap().to_string();
    //seq.qual1 = std::str::from_utf8(record.qual()).unwrap().to_string();

    tx.send(seq).expect("Sending seq object to writer");
  }
}

fn read_sam(tx:std::sync::mpsc::Sender<FastenSeq>, matches:&getopts::Matches){
  if matches.opt_present("paired-end") {
    logmsg("--paired-end given but paired-endedness will be determined by sam format flags");
  }

  // TODO check if sorted by name so that the pairs are next to each other

  let mut reader = bam::SamReader::from_path("/dev/stdin").unwrap();
  let mut r      = bam::Record::new();

  loop {
    let mut seq:FastenSeq = FastenSeq::new();

    match reader.read_into(&mut r) {
      Ok(false) => break,
      Ok(true)  => {},
      Err(e)    => panic!("{}", e),
    }
    seq.id1    = String::from(std::str::from_utf8(r.name()).unwrap());
    seq.seq1   = String::from_utf8(r.sequence().to_vec()).unwrap();
    seq.qual1  = String::from_utf8(r.qualities().to_readable()).unwrap();

    // Read from the sam itself whether this is paired end
    if r.flag().is_paired() {
      match reader.read_into(&mut r) {
        Ok(false) => break,
        Ok(true)  => {},
        Err(e)    => panic!("{}", e),
      }
      seq.id2    = String::from(std::str::from_utf8(r.name()).unwrap());
      seq.seq2   = String::from_utf8(r.sequence().to_vec()).unwrap();
      seq.qual2  = String::from_utf8(r.qualities().to_readable()).unwrap();
    }

    tx.send(seq).expect("Sending seq object to writer");
  }
}

fn read_fastq(tx:std::sync::mpsc::Sender<FastenSeq>, matches:&getopts::Matches){
  let paired_end = matches.opt_present("paired-end");

  let mut reader = bio::io::fastq::Reader::new(std::io::stdin());
  let mut record = bio::io::fastq::Record::new();

  loop{
    match reader.read(&mut record) {
      Ok(()) => {
        if record.is_empty() {
          break;
        }
      },
      Err(e)   => {panic!("{}",e);},
    };

    let mut seq:FastenSeq = FastenSeq::new();
    seq.id1   = record.id().to_string();
    seq.seq1  = std::str::from_utf8(record.seq()).unwrap().to_string();
    seq.qual1 = std::str::from_utf8(record.qual()).unwrap().to_string();

    if paired_end {
      match reader.read(&mut record) {
        Ok(()) => {
          if record.is_empty() {
            panic!("Specified paired end reads but there wasn't a pair for read {}", &seq.id1);
          }
        },
        Err(e)   => {panic!("{}",e);},
      };
      seq.id2   = record.id().to_string();
      seq.seq2  = std::str::from_utf8(record.seq()).unwrap().to_string();
      seq.qual2 = std::str::from_utf8(record.qual()).unwrap().to_string();
    }

    tx.send(seq).expect("Sending seq object to writer");

  }

}

fn write_fastq(rx:std::sync::mpsc::Receiver<FastenSeq>){
  let receiver = rx.iter();
  for seq in receiver {
    println!("{}", seq.as_fastq());
  }
}

fn write_fasta(rx:std::sync::mpsc::Receiver<FastenSeq>){
  let receiver = rx.iter();
  for seq in receiver {
    println!("{}", seq.as_fasta());
  }
}

fn write_sam(rx:std::sync::mpsc::Receiver<FastenSeq>){
  let receiver = rx.iter();
  for seq in receiver {
    println!("{}", seq.as_sam());
  }
}

