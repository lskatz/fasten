//! Normalizes kmer depth by removing some reads from high kmer depths
//! The input has to be from `fasten_kmer --remember-reads` where there are at least three columns:
//! kmer, count, read1, [read2,...]
//!
//! This was inspired by BBNorm and is probably not the exact same algorithm.
//! <https://jgi.doe.gov/data-and-tools/software-tools/bbtools/bb-tools-user-guide/bbnorm-guide/>

//! # Examples

//! ```bash
//! cat testdata/four_reads.fastq | \
//!   fasten_kmer -k 5 --remember-reads | \
//!   fasten_normalize | \
//!   gzip -c > four_reads.normalized.fastq.gz
//! ```
//!
//! Paired end reads
//! 
//! ```bash
//! cat testdata/R[12].fastq | \
//!   fasten_shuffle | \
//!   fasten_kmer -k 3 -m --paired-end | \
//!   fasten_normalize --target-depth 10 --paired-end | \
//!   gzip -c > normalized.fastq.gz
//! ```

//! # Usage

//! ```text
//! Usage: fasten_normalize [-h] [-n INT] [-p] [--verbose] [--version] [-t INT]
//!
//! Options:
//!     -h, --help          Print this help menu.
//!     -n, --numcpus INT   Number of CPUs (default: 1)
//!     -p, --paired-end    The input reads are interleaved paired-end
//!         --verbose       Print more status messages
//!         --version       Print the version of Fasten and exit
//!     -t, --target-depth INT
//!                         The target depth of kmer.
//! ```
    
extern crate fasten;
extern crate getopts;
extern crate rand;

use std::io::BufReader;
use std::io::BufRead;
use std::io::stdin;
use std::io::Stdin;
use std::cmp::min;
use rand::Rng;

use fasten::fasten_base_options;
use fasten::fasten_base_options_matches;
//use fasten::logmsg;

/// Glues together paired end reads internally and is a
/// character not expected in any read
const READ_SEPARATOR :char = '~';

fn main(){
    let mut opts = fasten_base_options();

    // script-specific options
    opts.optopt("t", "target-depth", "The target depth of kmer.", "INT");
    let matches = fasten_base_options_matches("Normalizes reads based on kmer coverage.", opts);

    let target_depth :u32 = matches.opt_str("target-depth")
        .expect("need --target-depth")
        .parse()
        .expect("Convert target-depth to integer");

    let stdin = stdin();

    let paired_end = matches.opt_present("paired-end");
    
    normalize_coverage(stdin, target_depth, paired_end);
}

/// Normalize the coverage to a certain target and print as a fastq
fn normalize_coverage (stdin:Stdin, target_depth:u32, paired_end:bool) {
    // start off a random thing so that we can get random reads later on
    let mut rng = rand::thread_rng();

    // read the file
    let my_buffer=BufReader::new(stdin);
    let mut buffer_iter = my_buffer.lines();
    while let Some(line_opt) = buffer_iter.next() {
        let line = line_opt.expect("read the next line");

        // get the fields: kmer, count, reads...
        let mut f :Vec<&str> = line.split("\t").collect();
        // No need to normalize if there are no reads and therefore nothing in field 3
        if f.len() < 3 {
            continue;
        }

        let kmer_count :Vec<&str> = f.splice(0..2, vec![]).collect();
        //let _kmer = kmer_count[0];
        let count :u32 = kmer_count[1].parse().unwrap();

        // number of reads to keep is the target depth / kmer coverage * number of reads present
        let mut num_reads_to_keep :usize = min(
            (target_depth as f32 / count as f32 * f.len() as f32).ceil() as usize,
            f.len() as usize
        ) as usize;
        if paired_end {
            num_reads_to_keep = (num_reads_to_keep as f32 / 2.0).ceil() as usize;
        }

        //println!("target depth:{} count:{} num reads:{} = {}", target_depth, count, f.len(), num_reads_to_keep);
        
        // shuffle the reads in place
        rng.shuffle(&mut f);
        // take the top X reads
        let reads_to_keep :Vec<&str> = f.splice(0..num_reads_to_keep, vec![]).collect();

        print_reads(reads_to_keep);
    }
}

/// Print the reads in fastq format when given in a single line with `~`
fn print_reads (reads:Vec<&str>) {
    for entry in reads{
        let entry_string = entry.replace(READ_SEPARATOR, "\n");
        println!("{}", entry_string);
    }
}

