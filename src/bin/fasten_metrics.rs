//! Gives read metrics on a read set.
//! Values are given in a column delimited stdout.

//! # Examples

//! ```bash
//! cat testdata/four_reads.fastq | fasten_metrics | column -t
//! ```

//! # Usage
//! ```text

//! Usage: fasten_metrics [-h] [-n INT] [-p] [-v] [--each-read]

//! Options:
//!    -h, --help          Print this help menu.
//!    -n, --numcpus INT   Number of CPUs (default: 1)
//!    -p, --paired-end    The input reads are interleaved paired-end
//!    -v, --verbose       Print more status messages
//!        --each-read     Print the metrics for each read. This creates very
//!                        large output
//! ```
    
extern crate fasten;
extern crate statistical;
extern crate getopts;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;
//use std::f32;
use std::f64;

use fasten::fasten_base_options;
use fasten::fasten_base_options_matches;
use fasten::logmsg;

#[test]
fn test_average_quality () {
    let easy_qual = "IIIIIIIIIIII";
    let easy_avg_obs = average_quality(easy_qual);
    let easy_avg_exp:f64 = 40.0;
    assert_eq!(easy_avg_obs, easy_avg_exp, "Tried to calculate average quality for {}", easy_qual);

    // a more difficult qual is the one in the first read in four_reads.fastq
    let hard_qual = "8AB*2D>C1'02C+=I@IEFHC7&-E5',I?E*33E/@3#68B%\"!B-/2%(G=*@D052IA!('7-*$+A6>.$89,-CG71=AGAE3&&#=2B.+I<E";
    let hard_avg_obs = average_quality_from_cigar(hard_qual);
    let hard_avg_exp:f64 = 21.40;
    assert_eq!(hard_avg_obs, hard_avg_exp, "Tried to calculate the average quality for {}", hard_qual);
}

fn main(){
    let mut opts = fasten_base_options();

    // script-specific options
    opts.optflag("","each-read","Print the metrics for each read. This creates very large output");

    let matches = fasten_base_options_matches("Gives read metrics on a read set.", opts);

    if matches.opt_present("paired-end") {
        logmsg("WARNING: --paired-end is not utilized in this script");
    }

    let each_read :bool=matches.opt_present("each-read");

    let filename = "/dev/stdin";
    
    // Header
    if each_read {
        each_read_metrics(&filename);
    } else {
        summary_metrics(&filename);
    }
}

const BUF_CAPACITY: usize = 256 * 1024;

/// Lookup table: quality byte (phred+33) => error probability
fn prob_table(ascii_base: u8) -> [f64; 256] {
    let mut table = [0.0f64; 256];
    for (q, slot) in table.iter_mut().enumerate() {
        let phred = (q as i16 - ascii_base as i16).max(0);
        *slot = 10_f64.powf(-(phred as f64) / 10.0);
    }
    table
}

/// Reads one line into buf without the trailing newline. Returns false at EOF.
fn next_line<R: BufRead>(reader: &mut R, buf: &mut Vec<u8>) -> bool {
    buf.clear();
    let n = reader.read_until(b'\n', buf).expect("Could not read line");
    if n == 0 {
        return false;
    }
    if buf.last() == Some(&b'\n') {
        buf.pop();
    }
    if buf.last() == Some(&b'\r') {
        buf.pop();
    }
    true
}

fn prob_to_phred(sum: f64, n: u64) -> f64 {
    if n == 0 {
        return 0.0;
    }
    -10.0 * (sum / n as f64).log10()
}

fn each_read_metrics(filename: &str) {
    let table = prob_table(33);
    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    writeln!(out, "readID\treadLength\tavgQual").unwrap();

    let my_file = File::open(&filename).expect("Could not open file");
    let mut reader = BufReader::with_capacity(BUF_CAPACITY, my_file);
    let mut buf: Vec<u8> = Vec::new();
    let mut id: Vec<u8> = Vec::new();
    let mut length: usize = 0;
    let mut num_lines: u64 = 0;

    while next_line(&mut reader, &mut buf) {
        num_lines += 1;
        match num_lines % 4 {
            1 => {
                id.clear();
                if !buf.is_empty() {
                    id.extend_from_slice(&buf[1..]);
                }
            }
            2 => length = buf.len(),
            0 => {
                let sum: f64 = buf.iter().map(|&q| table[q as usize]).sum();
                out.write_all(&id).unwrap();
                writeln!(out, "\t{}\t{:.2}", length as f64, prob_to_phred(sum, buf.len() as u64)).unwrap();
            }
            _ => {}
        }
    }
}

fn summary_metrics(filename: &str) {
    println!("totalReadLength\tnumReads\tavgReadLength\tavgQual");

    let table = prob_table(33);
    let mut total_length: u64 = 0;
    let mut total_qual_len: u64 = 0;
    let mut prob_sum: f64 = 0.0;
    let mut num_lines: u64 = 0;

    let my_file = File::open(&filename).expect("Could not open file");
    let mut reader = BufReader::with_capacity(BUF_CAPACITY, my_file);
    let mut buf: Vec<u8> = Vec::new();

    while next_line(&mut reader, &mut buf) {
        num_lines += 1;
        match num_lines % 4 {
            2 => total_length += buf.len() as u64,
            0 => {
                prob_sum += buf.iter().map(|&q| table[q as usize]).sum::<f64>();
                total_qual_len += buf.len() as u64;
            }
            _ => {}
        }
    }
    let num_reads: u64 = num_lines / 4;

    let summary = vec![
        (total_length as f64).to_string(),
        num_reads.to_string(),
        (total_length as f64 / num_reads as f64).to_string(),
        format!("{:.2}", prob_to_phred(prob_sum, total_qual_len)),
    ];
    println!("{}", summary.join("\t"));
}
