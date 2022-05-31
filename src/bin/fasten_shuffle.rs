//! Interleaves reads from either stdin or file parameters.
//! 
//! Many fasten executables are aware of paired end reads
//! but they need to be in interleaved format.
//! This script transforms R1 and R2 reads into interleaved format.
//! 
//! # Examples
//! 
//! ## Shuffling
//! 
//! ### Simple transformation of R1 and R2 into interleaved
//! ```bash
//! cat file_1.fastq file_2.fastq | fasten_shuffle > interleaved.fastq
//! fasten_shuffle -1 file_1.fastq -2 file_2.fastq > interleaved.fastq
//! ```
//! ### interleave R1 and R2 and pipe it into another executable with --paired-end
//! ```bash
//! cat file_1.fastq file_2.fastq | fasten_randomize --paired-end | head -n 8 > random-pair.fastq
//! ```
//! ### ... or to another executable with --paired-end
//! ```bash
//! cat file_1.fastq file_2.fastq | fasten_sample --paired-end --frequency 0.2 > downsample.20percent.fastq
//! ```
//! 
//! ## Deshuffling
//! 
//! ```bash
//! cat interleaved.fastq | fasten_shuffle -d -1 1.fastq -2 2.fastq
//! ```
//! 
//! # Usage
//! 
//! ```text
//! Usage: fasten_shuffle [-h] [-n INT] [-p] [-v] [-d] [-1 1.fastq] [-2 2.fastq]
//! 
//! Options:
//!     -h, --help          Print this help menu.
//!     -n, --numcpus INT   Number of CPUs (default: 1)
//!     -p, --paired-end    The input reads are interleaved paired-end
//!     -v, --verbose       Print more status messages
//!     -d, --deshuffle     Deshuffle reads from stdin
//!     -1 1.fastq          Forward reads. If deshuffling, reads are written to
//!                         this file.
//!     -2 2.fastq          Forward reads. If deshuffling, reads are written to
//!                         this file.
//! ```

extern crate getopts;
extern crate fasten;
use std::fs::File;
use std::io::Write;
use std::io::BufReader;

use fasten::fasten_base_options;
use fasten::fasten_base_options_matches;
use fasten::io::fastq;
use fasten::io::seq::Cleanable;
use fasten::io::seq::Seq;
use fasten::logmsg;

fn main(){
    let mut opts = fasten_base_options();
    //script-specific flags
    opts.optflag("d","deshuffle","Deshuffle reads from stdin");
    opts.optopt("1","","Forward reads. If deshuffling, reads are written to this file.","1.fastq");
    opts.optopt("2","","Forward reads. If deshuffling, reads are written to this file.","2.fastq");

    let matches = fasten_base_options_matches("Interleaves reads from either stdin or file parameters", opts);
    
    if matches.opt_present("paired-end") {
        logmsg("WARNING: --paired-end was supplied but it is assumed for this script anyway");
    }

    if matches.opt_present("deshuffle") {
        deshuffle(&matches);
    } else {
        shuffle(&matches);
    }
}

/// Read from stdin and deshuffle reads into files
fn deshuffle(matches: &getopts::Matches) -> () {
    
    // Where are we reading to?  Get those filenames.
    let r1_filename = if matches.opt_present("1") {
        matches.opt_str("1").unwrap()
    } else {
        "/dev/stdout".to_string()
    };
    let r2_filename = if matches.opt_present("2") {
        matches.opt_str("2").unwrap()
    } else {
        "/dev/stdout".to_string()
    };

    let mut file1 = File::create(r1_filename).expect("ERROR: could not write to file");
    let mut file2 = File::create(r2_filename).expect("ERROR: could not write to file");

    // read stdin
    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let fastq_reader=fastq::FastqReader::new(my_buffer);
    let mut read_counter=0;
    for seq in fastq_reader {
        
        // print to file 1 and to file 2, alternating each Seq
        if read_counter % 2 == 0 {
            write!(file1,"{}\n",seq.to_string()).unwrap();
        } else {
            write!(file2,"{}\n",seq.to_string()).unwrap();
        }
        read_counter+=1;
    }

}

/// Read fastq from stdin and interleave
fn shuffle(matches: &getopts::Matches) -> () {

    // Where are we reading from?  Get those filenames.
    let r1_filename = if matches.opt_present("1") {
        matches.opt_str("1").unwrap()
    } else {
        "/dev/stdin".to_string()
    };
    let r2_filename = if matches.opt_present("2") {
        matches.opt_str("2").unwrap()
    } else {
        "/dev/stdin".to_string()
    };

    // Read 1 first, and read 2 is halfway down.
    // Unfortunately this means that it all goes into ram.
    let mut seqs1 = read_seqs(&r1_filename);
    let mut seqs2 = read_seqs(&r2_filename);
    let mut num_pairs = seqs1.len();

    // If reading R1 from stdin, it is possible that seqs2 
    // is empty. If so, redistribute half the reads from 
    // seqs1 into seqs2.
    if seqs2.len() == 0 {
        num_pairs = ((num_pairs as f32)/2.0).ceil() as usize;
        // put it all into seqs_all and truncate the seqs
        let seqs_all = seqs1.clone();
        seqs1 = vec![];
        let mut seq_idx = 0;
        while seq_idx < num_pairs {
            if seq_idx + num_pairs >= seqs_all.len()-1 {
                logmsg("Looks like one of the R2 reads is missing. Skipping an R1/R2 pair.");
                logmsg("If this is in error, please see fasten_validate --paired-ends");
                break;
            }
            seqs1.push(seqs_all[seq_idx].clone());
            seqs2.push(seqs_all[seq_idx + num_pairs].clone());
            seq_idx += 2;
        }
        // Free up some memory
        drop(seqs_all);
    }

    for i in  0..num_pairs  {
        seqs1[i].print();
        seqs2[i].print();
    }

}

/// Read fastq entries from a filename
fn read_seqs(filename: &String) -> Vec<Seq> {

    let my_file = File::open(&filename).expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let fastq_reader=fastq::FastqReader::new(my_buffer);
    let seqs :Vec<Seq> = fastq_reader.collect();
    return seqs;
}
