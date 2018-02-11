#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]

extern crate ross;
extern crate statistical;
extern crate getopts;

use std::fs::File;
use std::io::BufReader;
use ross::io::fastq;
//use statistical::mean;

use std::sync::mpsc;
use std::thread;

use std::env;
use getopts::Options;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    // ROSS flags.
    // TODO put these options into ROSS to streamline.
    opts.optflag("h", "help", "Print this help menu.");
    opts.optopt("n","numcpus","Number of CPUs (default: 1)","INT");
    // Options specific to this script
    opts.optopt("s", "sample", "Only accept a frequency of reads, between 0 and 1 (default: 1)", "FLOAT");
    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");

    if matches.opt_present("h") {
        println!("{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let filename = "/dev/stdin";

    // receiving threads
    let (tx,rx)=mpsc::channel();

    // Read the file in a thread.
    let reader_handle = thread::spawn(move || {
      let mut num_reads=0;

      let my_file = File::open(&filename).expect("Could not open file");
      let my_buffer=BufReader::new(my_file);
      let mut fastq_reader=fastq::FastqReader::new(my_buffer);

      let mut read_length :Vec<f32> = Vec::new();
      let mut avg_qual :Vec<f32> = Vec::new();
      let mut num_bases :f32 = 0.0;
      let mut min_read_length :f32 = std::f32::MAX;
      let mut max_read_length :f32 = 0.0;

      for seq_obj in fastq_reader {
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

