#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

extern crate ross;
extern crate statistical;

use std::fs::File;
use std::io::BufReader;
use ross::io::fastq;
//use statistical::mean;

use std::sync::mpsc;
use std::thread;

fn main(){
    let filename = "/dev/stdin";

    // receiving threads
    let numcpus=1;
    let (tx,rx)=mpsc::channel();

    // Read the file in a thread.
    let reader_handle = thread::spawn(move || {
      let mut num_reads=0;

      let my_file = File::open(&filename).expect("Could not open file");
      let my_buffer=BufReader::new(my_file);
      let mut fastq_reader=fastq::Reader::new(my_buffer);

      let mut read_length :Vec<f32> = Vec::new();
      let mut avg_qual :Vec<f32> = Vec::new();
      let mut num_bases :f32 = 0.0;
      let mut min_read_length :f32 = std::f32::MAX;
      let mut max_read_length :f32 = 0.0;

      while let Some(seq_obj) = fastq_reader.read_carefully() {
        let mut abbr_entry = seq_obj.seq.clone();
        abbr_entry.push_str("\n");
        abbr_entry.push_str(&seq_obj.qual);
        tx.send(abbr_entry).expect("Could not send a String");
        num_reads+=1;
      }

      return num_reads;
    });

    // Analyze the fastq entries in a thread.
    let analysis_handle = thread::spawn(move || {
      let mut num_entries=0;
      for abbr_entry in rx{
        num_entries+=1;
      }
      return num_entries;
    });

    let num_reads=reader_handle.join();
    analysis_handle.join();
}

        /*
        // length of read
        let len = seq_obj.seq.len() as f32;
        read_length.push(len);
        num_bases += len;

        if len > max_read_length {
          max_read_length = len;
        }
        if len < min_read_length {
          min_read_length = len;
        }

        // quality
        let qual :Vec<f32> = seq_obj.qual.chars()
                                  .map(|x| {x as u8 as f32-33.0})
                                  .collect();
        avg_qual.push(mean(&qual));
        for received in rx {
        println!("{}", received);
      }
    return;

    
    println!("{}",vec![
        "read_length", "num_bases",
        "min_read_length", "max_read_length",
        "avg_quality", "num_reads", "PE?",
        "coverage",
    ].join("\t"));
    println!("{}",vec![
        mean(&read_length).to_string(),
        num_bases.to_string(),
        min_read_length.to_string(), max_read_length.to_string(),
        mean(&avg_qual).to_string(), num_reads.to_string(), 
        "0".to_string(),
    ].join("\t"));

     */

