//! Trims reads using 0-based coordinates
//! 
//! # Examples
//! 
//! ## Adapters
//! 
//! ### Download the adapter files
//! 
//! ```bash
//! mkdir -pv $HOME/db
//! pushd $HOME/db # step into the db directory
//! git clone https://github.com/lskatz/adapterseqs
//! ADAPTERS=$(find $HOME/db/adapterseqs -name '*.fa')
//! popd # return to the original directory
//! ```
//! 
//! ### Trim the adapters
//! 
//! ```bash
//! cat file.fastq | fasten_trim 
//! ```
//! 
//! ## Blunt-end trim five bases from the right side
//! 
//! ```bash
//! cat file.fastq | fasten_trim -l -5 > trimmed.fastq
//! ```
//!
//! ## Keep a maximum of 100bp with blunt-end trimming on the right side
//! 
//! ```bash
//! cat file.fastq | fasten_trim -l 99 > trimmed.fastq
//! ```
//! 
//! ## Blunt-end trim 5bp from the left side
//! 
//! ```bash
//! cat file.fastq | fasten_trim -f 4  > trimmed.fastq
//! ```
//! 
//! # Usage
//! 
//! ```text
//! Usage: fasten_trim [-h] [-n INT] [-p] [-v] [-f INT] [-l INT]
//! 
//! Options:
//!     -h, --help          Print this help menu.
//!     -n, --numcpus INT   Number of CPUs (default: 1)
//!     -p, --paired-end    The input reads are interleaved paired-end
//!     -v, --verbose       Print more status messages
//!     -f, --first-base INT
//!                         The first base to keep (default: 0)
//!     -l, --last-base INT The last base to keep. (default: 0)
//! ```
//! 
//! # Notes
//! 
//! The algorithm is as follows:
//! 
//! 1. marks the first and last bases for trimming as 0 and the last base, respectively
//! 2. if an adapter is found at the beginning of the sequence, then move the marker for where it will be trimmed
//! 3. Compare the blunt end suggested trimming against where an adapter might be found and move the marker as the most inward possible
//! 4. Trim the sequence and quality strings
//! 
//! # Output
//! 
//! The deflines will be altered with a description of the trimming in brackets, e.g.,
//! [trimmed_adapter_rev=TT] [trimmed_left=0] [trimmed_right=250]

extern crate fasten;
extern crate statistical;
extern crate getopts;
extern crate threadpool;

use std::fs::File;
use std::io::BufReader;
use std::cmp::{min,max};
use std::process::exit;

use std::collections::HashMap;
use std::io::BufRead;

use fasten::fasten_base_options;
use fasten::fasten_base_options_matches;
use fasten::logmsg;
use fasten::reverse_complement;
use fasten::io::fastq;
use fasten::io::seq::Seq;

fn main(){
    let mut opts = fasten_base_options();

    // script-specific options
    opts.optopt("f","first-base","The first base to keep (default: 0)","INT");
    opts.optopt("l","last-base","The last base to keep (default: 0)","INT");
    opts.optopt("a","adapterseqs","fasta file of adapters","path/to/file.fa");

    let matches = fasten_base_options_matches("Blunt-end trims using 0-based coordinates", opts);

    let adapterseqs:String = {
        if matches.opt_present("adapterseqs") {
            matches.opt_str("adapterseqs")
                .expect("ERROR: could not understand parameter --adapterseqs")
        } else {
            "".to_string()
        }
    };

    // store the adapter sequences as a vector of strings
    let mut adapters:Vec<String> = Vec::new();
    if matches.opt_present("adapterseqs") && adapterseqs.len() > 0 {
        // check that the file path exists
        // if not, exit with an error
        if !std::path::Path::new(&adapterseqs).exists() {
            logmsg(format!("ERROR: adapter file {} does not exist", &adapterseqs));
            exit(1);
        }

        // read the adapter sequences from the fasta file
        adapters = read_fasta(&adapterseqs)
            .values()
            .map(|x| x.to_string())
            .collect();
    }
    
    //if matches.opt_present("verbose") { 
    //    //logmsg(&adapters); 
    //    eprintln!("Adapters: {:?}", adapters);
    //    exit(3); 
    //}

    let first_base:usize ={
        if matches.opt_present("first-base") {
            matches.opt_str("first-base")
                .expect("ERROR: could not understand parameter --first-base")
                .parse()
                .expect("ERROR: --first-base is not an INT")
        } else {
            0
        }
    };

    let last_base:usize ={
        if matches.opt_present("last-base") {
            matches.opt_str("last-base")
                .expect("ERROR: could not understand parameter --last-base")
                .parse()
                .expect("ERROR: --last-base is not an INT")
        } else {
            0
        }
    };

    let _num_cpus:usize = {
      if matches.opt_present("numcpus") {
        /*
        matches.opt_str("numcpus")
            .expect("ERROR: could not understand parameter --numcpus")
            .parse()
            .expect("ERROR: --numcpus is not an INT");
        */
        logmsg("Warning: multithreading this script currently slows it down. Resetting to 1 cpu.  Avoid this warning by not using --numcpus");
        1 as usize
      } else {
        1 as usize
      }
    };
    
    // Read from stdin
    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let fastq_reader = fastq::FastqReader::new(my_buffer);
    let fastq_iter  = fastq_reader.into_iter();
    for seq in fastq_iter {

        let trimmed:String = trim_worker(seq, first_base, last_base, &adapters);
        println!("{}", trimmed);
    }
}

/// Trim a set of fastq entries and send it to a channel
fn trim_worker(seq:Seq, suggested_first_base:usize, suggested_last_base:usize, adapters:&Vec<String> ) -> String {

    // In this function, keep track of where the first and
    // last base would be trimmed with a simple marker.
    // Most instances of the word "trimming" in this function is just moving first_base and last_base.
    let mut first_base = 0;
    // The last position is either the last_base parameter
    // or the last position in the string, whichever is less.
    let mut last_base = seq.seq.len()-1;

    // Make note of what is trimmed
    let mut description = String::new();

    // First, run the adapter trimming, before any blunt end trimming

    // First, detect if there are any adapters in the sequence
    // If there are, then trim the sequence at the adapter
    for adapter in adapters {
        let adapter_length = adapter.len();
        
        // If the adapter is longer than the sequence, skip it: it won't exist in the sequence as a whole adapter.
        if adapter_length > seq.seq.len() {
            continue;
        }
        
        // Check if the adapter is at the beginning of the sequence
        if &seq.seq[0..adapter_length] == adapter {
            first_base = adapter_length;
            description.push_str(&format!(" [trimmed_adapter_fwd={}]", &adapter));
        }
        
        // Check if the revcom is at the end of the sequence
        let revcom = reverse_complement(&adapter);
        let end_slice: &str = &seq.seq[&seq.seq.len()-1 - adapter_length..].trim();
        if end_slice == revcom {
            last_base = seq.seq.len() - adapter_length;
            description.push_str(&format!(" [trimmed_adapter_rev={}]", &revcom));
        }
    }

    // Next, run the blunt end trimming.
    // Take the maximum between the suggested left trim and the current left trim.
    // If the left trim is longer than the sequence length, then omit a warning and do not trim.
    first_base = max(first_base, suggested_first_base);
    if first_base >= seq.seq.len() {
        logmsg("Warning: the left trim is longer than the sequence length.  Skipping.");
        first_base = 0;
    }

    // Take the minimum between the suggested right trim and the current right trim.
    // If the last base is less than 1, then omit a warning and do not trim.
    last_base = {
        if suggested_last_base == 0 {
            last_base
        } else {
            min(last_base, suggested_last_base)
        }
    };
    if last_base < 1 {
        logmsg("Warning: the right trim is longer than the sequence length.  Skipping.");
        last_base = seq.seq.len()-1;
    }

    description.push_str(&format!(" [trimmed_left={}] [trimmed_right={}]", first_base, last_base));

    let sequence = &seq.seq[first_base..last_base];
    let quality  = &seq.qual[first_base..last_base];

    let trimmed = format!("{}{}\n{}\n+\n{}", seq.id, description, sequence, quality);
    return trimmed;
}

// Taken from https://medium.com/bioinformatics-with-rust/how-to-read-a-fasta-file-9472b77589f7
/// Read a fasta file and return a HashMap of the sequences
fn read_fasta(file_path: &str) -> HashMap<String, String> {
    let mut data = HashMap::new();
    let file = File::open(file_path).expect("Invalid filepath");
    let reader = BufReader::new(file);
    
    let mut seq_id = String::new();

    for line in reader.lines() {
        let line = line.unwrap();
        
        // Check if the line starts with '>' (indicating a sequence ID or header)
        if line.starts_with('>') {
            seq_id = line.trim_start_matches('>').to_string();
        } else {
            // If it's a DNA sequence line, insert or update the HashMap entry
            // If seq_id is not present, insert a new entry with an empty String
            // Then append the current line to the existing DNA sequence
            data.entry(seq_id.clone()).or_insert_with(String::new).push_str(&line);
        }
    }
    
    data
}

