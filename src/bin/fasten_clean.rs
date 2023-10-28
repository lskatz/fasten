//! Trim and filter reads
//! # Examples
//! 
//! ```bash
//! cat testdata/four_lines.fastq | \
//!   fasten_clean > out.fastq
//! ```
//! ## more options
//! ```bash
//! cat testdata | \
//!   fasten_clean --min-avg-quality 25 --min-trim-quality 25 \
//!   > out.fastq
//! 
//! ```
//! 
//! # Usage
//! ```text
//! Usage: fasten_clean [-h] [-n INT] [-p] [-v] [--min-length INT] [--min-avg-quality FLOAT] [--min-trim-quality INT]
//!
//!    Options:
//!        -h, --help          Print this help menu.
//!        -n, --numcpus INT   Number of CPUs (default: 1)
//!        -p, --paired-end    The input reads are interleaved paired-end
//!        -v, --verbose       Print more status messages
//!            --min-length INT
//!                            Minimum length for each read in bp
//!            --min-avg-quality FLOAT
//!                            Minimum average quality for each read
//!            --min-trim-quality INT
//!                            Trim the edges of each read until a nucleotide of at
//!                            least X quality is found
//! ```

extern crate getopts;
extern crate fasten;

use std::fs::File;
use std::io::BufReader;

use fasten::fasten_base_options;
use fasten::fasten_base_options_matches;
use fasten::io::fastq;
//use fasten::logmsg;

fn main(){
    let mut opts = fasten_base_options();

    // script-specific options
    opts.optopt("","min-length","Minimum length for each read in bp","INT");
    opts.optopt("","min-avg-quality","Minimum average quality for each read","FLOAT");
    opts.optopt("","min-trim-quality","Trim the edges of each read until a nucleotide of at least X quality is found","INT");

    let matches = fasten_base_options_matches("Trims and filters reads", opts);

    let mut min_length :usize = 0;
    if matches.opt_present("min-length") {
        min_length = matches.opt_str("min-length")
            .expect("ERROR: could not read the minimum length parameter")
            .parse()
            .expect("ERROR: min-length is not an integer");
    }

    let mut min_avg_qual :f32 = 0.0;
    if matches.opt_present("min-avg-quality") {
        min_avg_qual = matches.opt_str("min-avg-quality")
            .expect("ERROR: could not read the minimum average quality parameter")
            .parse()
            .expect("ERROR: min-avg-qual is not an integer");
    }

    let mut min_trim_qual :u8 = 0;
    if matches.opt_present("min-trim-quality") {
        min_trim_qual = matches.opt_str("min-trim-quality")
            .expect("ERROR: could not read the minimum trim quality parameter")
            .parse()
            .expect("ERROR: min-trim-qual is not an integer");
    }

    let paired_end:bool = matches.opt_present("paired-end");

    let _num_cpus:usize = {
        if matches.opt_present("numcpus") {
            matches.opt_str("numcpus").expect("ERROR parsing --numcpus")
                .parse()
                .expect("ERROR: numcpus is not an integer")
        } else {
            1
        }
    };

    // Read the file and send seqs to threads
    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let fastq_reader = fastq::FastqReader::new(my_buffer);
    let fastq_iter = fastq_reader.into_iter();

    let mut trimmed_ids  :Vec<String> = vec![];
    let mut trimmed_seqs :Vec<String> = vec![];
    let mut trimmed_quals:Vec<String> = vec![];
    for seq in fastq_iter {
        // trim
        let (seq_trimmed, qual_trimmed) = trim(&seq.seq, &seq.qual, min_trim_qual);
        trimmed_ids.push(seq.id);
        trimmed_seqs.push(seq_trimmed);
        trimmed_quals.push(qual_trimmed);

        if paired_end && trimmed_seqs.len() < 2 {
            continue;
        }

        // ensure that we pass min qual and length for all reads
        let mut passed = true;
        for q_str in &trimmed_quals {
            if avg_quality(&q_str) < min_avg_qual {
                passed = false;
                break;
            }
            if q_str.len() < min_length {
                passed = false;
                break;
            }
        }

        if passed {
            for i in 0..trimmed_seqs.len() {
                println!("{}\n{}\n+\n{}",
                         &trimmed_ids[i],
                         &trimmed_seqs[i],
                         &trimmed_quals[i],
                         );
            }
        }

        // Before we end the loop, we still need to clear our sequence
        // buffer.
        trimmed_ids.clear();
        trimmed_seqs.clear();
        trimmed_quals.clear();
    }

}

/// Determine average quality of a qual cigar string,
/// e.g., let q:f32 = avg_quality("AABC!...")
fn avg_quality(qual: &String) -> f32 {
    let mut total :u32 = 0;
    for qual_char in qual.chars() {
        total += qual_char as u8 as u32;
    }
    let avg = (total as f32 / qual.len() as f32) - 33.0;
    return avg;
}

/// Trim the ends of reads with low quality
fn trim(seq: &String, qual: &String, min_qual: u8) -> (String,String) {
    let mut trim5 :usize=0;
    let mut trim3 :usize=qual.len();

    let offset_min_qual = min_qual + 33;
    
    // 5'
    for qual in qual.chars(){
        if (qual as u8) < offset_min_qual {
            trim5+=1;
        } else {
            break;
        }
    }

    // 3'
    for qual in qual.chars().rev() {
        if (qual as u8) < offset_min_qual {
            trim3-=1;
        } else {
            break;
        }
    }

    let new_seq :String;
    let new_qual:String;
    
    if trim5 >= trim3 {
        new_seq = String::new();
        new_qual= String::new();
    } else {
        new_seq  =  seq[trim5..trim3].to_string();
        new_qual = qual[trim5..trim3].to_string();
    }
    return(new_seq,new_qual);
}

