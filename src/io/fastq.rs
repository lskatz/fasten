use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use io::seq::Seq;
use io::seq::Cleanable;

use regex::Regex;

#[test]
/// Test whether we can read the test data carefully.
fn test_the_careful_reader () {
    let my_file = File::open("testdata/four_reads.fastq").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut fastq_reader=Reader::new(my_buffer);
    let seq_obj = fastq_reader.read_carefully().expect("Could not read four_reads.fastq");
    assert_eq!(seq_obj.seq.trim(), "AAAGTGCTCTTAACTTGTCCCGCTCCACATCAGCGCGACATCAATCGACATTAAACCGAGTATCTTGTCAGCCTGGGGTGACGATGCGTCCCATTAAAGT");
}

#[test]
/// Test whether we can read the test data quickly.
fn test_the_fast_reader () {
    let my_file = File::open("testdata/four_reads.fastq").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut fastq_reader=Reader::new(my_buffer);
    let seq_obj = fastq_reader.read_quickly().expect("Could not read four_reads.fastq");
    assert_eq!(seq_obj.seq.trim(), "AAAGTGCTCTTAACTTGTCCCGCTCCACATCAGCGCGACATCAATCGACATTAAACCGAGTATCTTGTCAGCCTGGGGTGACGATGCGTCCCATTAAAGT");
}

/// A FastQ reader
pub struct Reader<R: io::Read> {
  reader:           io::BufReader<R>,
}

impl<R: io::Read> Reader<R>{
  pub fn new(reader: R) -> Self {
    Reader {
      reader: io::BufReader::new(reader),
    }
  }

  /// An alias for self.read_quickly().
  pub fn read(&mut self) -> Option<Seq> {
    return self.read_quickly();
  }

  /// Read a fastq entry but assume that there are only
  /// four lines per entry (id, seq, plus, qual).
  pub fn read_quickly(&mut self) -> Option<Seq> {
    let mut id=    String::new();
    let mut seq=   String::new();
    let mut qual=  String::new();

    // Read the ID of the entry
    match self.reader.read_line(&mut id) {
        Ok(n) => {
            if n < 1 {
                return None;
            }
        }
        Err(error) => {
            panic!("ERROR: could not read ID line: {}",error);
        }
    }

    self.reader.read_line(&mut seq).expect("ERROR: could not read sequence line");
    
    // burn the plus sign
    let mut _plus = String::new();
    self.reader.read_line(&mut _plus).expect("ERROR: plus sign line not found");

    self.reader.read_line(&mut qual).expect("ERROR: could not read qual line");

    let seq = Seq::new(&id, &seq, &qual);

    Some(seq)
  }

  /// Read a fastq entry in the most correct way possible,
  /// allowing for whitespace in seq and qual lines.
  pub fn read_carefully(&mut self) -> Option<Seq> {
    let whitespace_regex = Regex::new(r"(\s+)").expect("malformed regex");

    let mut id=    String::new();
    let mut seq=   String::new();
    let mut qual=  String::new();

    // Read the ID of the entry
    match self.reader.read_line(&mut id) {
        Ok(n) => {
            // if we're expecting an ID line, but
            // there are zero bytes read, then we are
            // at the end of the file. Break.
            if n < 1 {
                return None;
            }
        }
        Err(error) => {
            panic!("ERROR: {}",error);
        }
      
    }
    // Read the DNA line of the entry and count
    // how long it is.
    'dna: loop{
        let mut buf = String::new();
        match self.reader.read_line(&mut buf) {
            Ok(n) => {
                if n < 1 {
                    panic!("ERROR: incomplete entry (no seq line), seqid {}\nbuf {}", id.trim(),buf);
                }
                // if we hit the qual line, then it is a single
                // character, +
                else if &buf[0..1] == "+" {
                    break 'dna;
                }
                else {
                    seq.push_str(&buf);
                }
            }
            Err(error) => {
                panic!("ERROR while reading seq for ID {}: {}",id.trim(),error);
            }
        }
    }
    // remove all whitespace
    seq = whitespace_regex.replace_all(&seq,"").into_owned();
    let read_length :usize=seq.len(); 
    
    // build onto the qual line until it has the right
    // number of bytes.
    'qual: loop{
        let mut buf = String::new();
        match self.reader.read_line(&mut buf) {
            Ok(n) => {
                if n < 1 {
                    panic!("ERROR: incomplete entry (no qual line), seqid {}\nbuf {}", id.trim(),buf);
                }
                else {
                  qual.push_str(&buf);
                }
                qual = whitespace_regex.replace_all(&qual,"").into_owned();
                if qual.len() >= read_length {
                  break 'qual;
                }
            }
            Err(error) => {
                panic!("ERROR while reading qual for ID {}: {}",buf.trim(),error);
            }
        }
    }

    let seq = Seq::new(&id, &seq, &qual);
    Some(seq)
  }
}

