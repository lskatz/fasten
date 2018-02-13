extern crate ross;
extern crate statistical;
extern crate getopts;

use std::fs::File;
use std::io::BufReader;
use ross::io::fastq;
use ross::io::seq::Seq;
use ross::io::seq::Cleanable;
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

    let mut numcpus :u8 = 1; // default: 1
    if matches.opt_present("numcpus") {
        numcpus = str::parse::<u8>( 
            &matches.opt_str("numcpus")
            .expect("ERROR: numcpus was not supplied")
            ).expect("ERROR: could not convert numcpus to u8 type");
    }

    // receiving threads
    let (tx,rx)=mpsc::channel();

    // Read the file in a thread.
    //let mut reader_handle=Vec::new();
    let reader_handle = thread::spawn(move || {
      let mut num_reads=0;

      let my_file = File::open(&filename).expect("Could not open file");
      let my_buffer=BufReader::new(my_file);
      let fastq_reader=fastq::FastqReader::new(my_buffer);

      for seq_obj in fastq_reader {
        let mut abbr_entry = seq_obj.to_string();
        abbr_entry.push_str("\n");
        tx.send(abbr_entry).expect("Could not send a String");
        num_reads+=1;
      }

      return num_reads;
    });

    // Analyze the fastq entries in a thread.
    println!("{}",vec!["avgReadLength","avgQual"].join("\t"));
    let analysis_handle = thread::spawn(move || {
        let mut num_entries=0;
        let mut total_length=0;
        let mut total_qual=0;
        for abbr_entry in rx{
            let seq = Seq::from_string(&abbr_entry);
            num_entries+=1;
            total_length+=seq.seq.len();
            for qual_char in seq.qual.chars() {
                let qual_int = qual_char as usize -33;
                total_qual = total_qual + qual_int;
            }
        }
        println!("{}", vec![
                 (total_length as f32/num_entries as f32).to_string(),
                 (total_qual as f32/total_length as f32).to_string(),
        ].join("\t"));

        return num_entries;
    });

    reader_handle.join()
        .expect("ERROR: could not join the reader handle");
    analysis_handle.join()
        .expect("ERROR: could not join the analysis handle");
}

