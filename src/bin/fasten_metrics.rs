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
        println!("readID\treadLength\tavgQual");
    } else {
        println!("{}",vec!["totalLength", "numReads", "avgReadLength","avgQual"].join("\t"));

    }
    
    let mut read_length :Vec<f64> = vec![];
    let mut read_qual   :Vec<u8>  = vec![];
    let mut num_lines   :u64   =0;

    // read the file
    let my_file = File::open(&filename).expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    for line in my_buffer.lines() {
        num_lines+=1;
        let mod_line = num_lines % 4;

        match mod_line {
            1 => {
                if each_read {
                    let id = line.expect("Expected an ID line");
                    // remove the @
                    print!("{}\t",&id[1..]);
                }
            }
            2 => {
                let my_read_length=line.expect("Expected a sequence line").len() as f64;
                if each_read {
                    print!("{}\t",my_read_length);
                }
                read_length.push(my_read_length);
            }
            0 => {
                let qual_line=line.expect("Expected a qual line");
                
                let my_qual_vec: Vec<u8> = qual_line.into_bytes();
                // TODO this if statement makes the program take twice as long. Optimize?
                if each_read {
                    let my_avg_qual = avg_qual(&my_qual_vec, 33);
                    println!("{:.2}",my_avg_qual);
                }
                read_qual.extend(my_qual_vec.into_iter());

            }
            _ => {

            }
        };
    }
    let num_reads :u64 = num_lines / 4;
    let total_length = read_length.iter().fold(0.0,|a,&b| a+b);

    let mut summary_metrics=vec![total_length.to_string(),num_reads.to_string()];

    // add statistics if requested
    let total_length_str = (total_length as f64/num_reads as f64).to_string();
    let total_qual_str = format!("{:.2}", avg_qual(&read_qual, 33));

    summary_metrics.push(total_length_str);
    summary_metrics.push(total_qual_str.to_string());

    // summary metrics
    if !each_read {
        println!("{}", summary_metrics.join("\t"));
    }

}

/// Calculates average quality value from a vector of quality bytes
fn avg_qual(qual: &[u8], ascii_base: u8) -> f64 {
    if qual.is_empty() {
        return 0.0;
    }
    
    // Parse quality values (assuming Phred scores)
    let qual_values: Vec<f64> = qual.iter()
        .map(|&q| {
            let phred = (q - ascii_base) as i16;
            // Convert Phred to probability: P = 10^(-Q/10)
            10_f64.powf(-phred as f64 / 10.0)
        })
        .collect();
    
    if qual_values.is_empty() {
        return 0.0;
    }
    
    let sum: f64 = qual_values.iter().sum();
    let avg_prob = sum / qual_values.len() as f64;
    
    // Convert average probability back to Phred score
    -10.0 * avg_prob.log10()
}


