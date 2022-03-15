//! Collapse identical reads into single reads, recalculating quality values.
//! If paired end, then each set of reads must be identical to be collapsed.
//! _Warning_: due to multiple reads collapsing into one, read identifiers will be reconstituted.

//! # Examples

//! ```bash
//! cat testdata/four_reads | fasten_combine > combined.fastq
//! ```
//!## Usage
//!
//!```text
//!Usage: fasten_combine [-h] [-n INT] [-p] [-v] [--max-qual-char CHAR] [--min-qual-char CHAR]
//!
//!Options:
//!    -h, --help          Print this help menu.
//!    -n, --numcpus INT   Number of CPUs (default: 1)
//!    -p, --paired-end    The input reads are interleaved paired-end
//!    -v, --verbose       Print more status messages
//!        --max-qual-char CHAR
//!                        Maximum quality character (default: I)
//!        --min-qual-char CHAR
//!                        Minimum quality character (default: !)
//!
//!    NOTE: range of quality scores is !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHI
//!```

extern crate fasten;
extern crate fastq;
extern crate getopts;
extern crate rand;

use std::io::stdin;
//use std::fs::File;
//use std::io::BufReader;
//use std::io::BufRead;
use std::collections::HashMap;

use std::env;
use std::f32;

use fasten::fasten_base_options;
use fastq::{Parser, Record};
use fasten::logmsg;

/// need this constant because the compiler had a problem
/// with the syntax `10.0.pow()`
const TEN: f32 = 10.0;
/// Glues together paired end reads internally and is a
/// character not expected in any read
const READ_SEPARATOR :char = '~';

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = fasten_base_options();

    // make a string of characters like !"#...GHI to represent all quals
    let default_phred_min_char:char = '!';
    let default_phred_max_char:char = 'I';
    let default_phred_min:u8 = default_phred_min_char as u8 - 33;
    let default_phred_max:u8 = default_phred_max_char as u8 - 33;
    let mut qual_range_string = String::with_capacity((default_phred_max - default_phred_min + 1) as usize);
    for phred in default_phred_min..default_phred_max+1 {
      qual_range_string.push((phred+33) as char);
    }


    opts.optopt("","max-qual-char",
      format!("Maximum quality character (default: {})", default_phred_max_char).as_str(),
      "CHAR");
    opts.optopt("","min-qual-char",
      format!("Minimum quality character (default: {})", default_phred_min_char).as_str(),
      "CHAR");

    let matches = opts.parse(&args[1..]).expect("Parsing parameters");

    if matches.opt_present("h") {
        println!("Collapse identical reads into single reads, recalculating quality values. If paired end, then each set of reads must be identical to be collapsed. Warning: due to multiple reads collapsing into one, read identifiers will be reconstituted.");
        println!("{}",opts.usage(&opts.short_usage(&args[0])));
        println!("NOTE: range of quality scores is {}", qual_range_string);
        std::process::exit(0);
    }
    
    let max_qual_char:char = matches.opt_default("max-qual-char", &default_phred_max_char.to_string())
                     .unwrap_or(String::from(default_phred_max_char))
                     .parse()
                     .expect("ERROR converting --max-qual-int value to integer");

    let mut min_qual_char:char =
          matches.opt_default("min-qual-char", &default_phred_min_char.to_string())
                     .unwrap_or(String::from(default_phred_min_char))
                     .parse()
                     .expect("ERROR converting --min-qual-int value to integer");
    if min_qual_char < default_phred_min_char {
      logmsg("--min-qual-char was less than the default minimum and so it will be set to the default");
      min_qual_char = default_phred_min_char;
    }

    // Finally turn the choice of qual into an integer
    let min_qual:u8 = min_qual_char as u8;
    let max_qual:u8 = max_qual_char as u8;

    let paired_end = matches.opt_present("paired-end");
    let _num_cpus:usize = {
      if matches.opt_present("numcpus") {
        logmsg("Warning: This script does not make use of --numcpus");
        1 as usize
      } else {
        1 as usize
      }
    };

    // seq => count
    let mut seq_count :HashMap<String, u32>   =HashMap::new();
    // seq => vec![sequence of prob of errors]
    let mut seq_error_rate :HashMap<String, Vec<f32>> = HashMap::new();

    let parser = Parser::new(stdin());
    let mut parser_getter = parser.ref_iter();
    parser_getter.advance().expect("Could not read the first fastq entry");
    while let Some(record1) = parser_getter.get() {
        let mut id:Vec<u8>     = record1.head().to_vec();
        let mut seq:Vec<u8>    = record1.seq().to_vec(); 
        let mut qual:Vec<u8>   = record1.qual().to_vec();
        if paired_end {
          // get the next entry with advance() and then get()
          match parser_getter.advance() {
            Ok(_) => {},
            Err(err) => {
              panic!("ERROR: could not read the second entry in a paired end read: {}", err);
            }
          };
          let record2 = &parser_getter.get().expect("ERROR parsing second pair in a paired end read");
          let id2:&[u8]  = record2.head();
          let seq2:&[u8] = record2.seq();
          let qual2:&[u8]= record2.qual();

          // Add on the separator
          id.push(READ_SEPARATOR as u8);
          seq.push(READ_SEPARATOR as u8);
          qual.push(READ_SEPARATOR as u8);
          
          // Add on the second read
          id.extend_from_slice(id2);
          seq.extend_from_slice(seq2);
          qual.extend_from_slice(qual2);
        }
        //println!("{:?}", qual);

        // Keep track of the counts of identical sequence
        let seq_string:String = String::from(
                                   std::str::from_utf8(&seq[..])
                                   .expect("ERROR converting slice to str")
                                );
        /*
        let id_string:String = String::from(
                                  std::str::from_utf8(&id[..])
                                 .expect("ERROR converting slice to str")
                               );
        */
        let count = seq_count.entry(seq_string.clone()).or_insert(0);
        *count += 1 as u32;

        // If this sequence hasn't been seen yet,
        // then instantiate the probabilities.
        if !seq_error_rate.contains_key(&seq_string) {
            let mut qual_vec:Vec<f32> = vec![];
            for q in qual {
                // Don't mess with the read separator character
                if q == READ_SEPARATOR as u8 {
                  qual_vec.push(q as u8 as f32);
                  continue;
                }
                let qual_int = q as u8 as f32 - 33.0;
                let p :f32 = TEN.powf((-1.0 * qual_int)/TEN);
                qual_vec.push(p);
            }
            seq_error_rate.insert(seq_string.clone(), qual_vec);
        }
        //println!("{:?}", seq_error_rate.entry(seq_string));
        
        // If this sequence has been seen yet, then
        // start combining the error rates.
        else {
            // get the base error rate vector
            let qual_vec = seq_error_rate.entry(seq_string.clone()).or_insert(Vec::new());

            let these_errors = qual.into_iter().map(|qual_char|{
                // Don't mess with the read separator character
                if qual_char == READ_SEPARATOR as u8 {
                  return qual_char as u8 as f32;
                }
                let qual_int = qual_char as u8 as f32 - 33.0;
                let p :f32 = TEN.powf((-1.0 * qual_int)/TEN);
                return p;
            }).collect();
            let new_qual = combine_error_vectors(&qual_vec,&these_errors);
            *qual_vec = new_qual;
        }

        match &parser_getter.advance() {
          Ok(_) => {},
          Err(_) => {break;}
        };
    }

    let mut seq_counter=0;
    for (seq,combined_qual) in seq_error_rate {
        seq_counter += 1;
        //println!("{:?}", seq);continue;

        // TODO take care of PE reads

        // Make a new cigar line for quality
        let mut qual_cigar = String::new();
        for p in combined_qual {
            let mut qual_recalc :f32 = -TEN * (p).log(TEN)+33.0;
            // check for overflow before switching to u8
            if qual_recalc.is_infinite() || qual_recalc > max_qual as f32 {
                qual_recalc = max_qual as f32;
            }

            // switch to u8 and then the corresponding char
            let mut qual_recalc_char = qual_recalc.round() as u8 as char;
            if (qual_recalc_char as u8) > max_qual {
                qual_recalc_char = max_qual_char;
            }
            // a reduction in quality is not expected... but just in case.
            if (qual_recalc_char as u8) < min_qual {
                qual_recalc_char = min_qual_char;
            }
            qual_cigar.push(qual_recalc_char);
        }

        if paired_end {
            // split the seq and qual for paired end
            let separator_pos = seq.find(READ_SEPARATOR).expect("ERROR finding read separator");
            let r1_seq = seq[0..separator_pos].to_string();
            let r2_seq = seq[separator_pos+1..].to_string();
            let r1_qual= qual_cigar[0..separator_pos].to_string();
            let r2_qual= qual_cigar[separator_pos+1..].to_string();
            println!("@{}/1\n{}\n+\n{}",seq_counter,r1_seq,r1_qual);
            println!("@{}/2\n{}\n+\n{}",seq_counter,r2_seq,r2_qual);
        } else {
            println!("@{}\n{}\n+\n{}",seq_counter,seq,qual_cigar);
        }
    }
}

/// Combines vectors of error probabilities
/// such that the rate of error is probability of error
/// from vector one times the probability of error 
/// from vector two.
fn combine_error_vectors(errors1 :&Vec<f32>, errors2: &Vec<f32>) -> Vec<f32> {
    if errors1.len() != errors2.len() {
        panic!("Lengths of error vectors do not match: {} and {}", errors1.len(), errors2.len());
    }
    let mut errors_iter2=errors2.iter();
    let mut new_errors :Vec<f32> = Vec::new(); // TODO set length/capacity to errors.len()
    for p1 in errors1 {
        let p2 = errors_iter2.next().expect("ERROR: could not get the error probability from the second read");
        new_errors.push(p1 * p2);
    }
    return new_errors;
}

// TODO a function that returns the 'min-seq' which is the
// sequence that comes first alphabetically when compared
// with its revcom

/*
#[allow(dead_code)]
fn recalculate_qual(qual_str: &str, count: u32) -> String {
    let mut qual_out = String::new();

    let max_qual = 'I' as u8;
    let min_qual = '!' as u8;

    let qual = qual_str.to_string();
    for qual_char in qual.chars() {
        let qual_int = qual_char as u8 as f32 - 33.0;
        //let ten:f32=10.0;
        let p :f32 = TEN.powf((-1.0 * qual_int)/TEN);
        let p_recalc :f32 = p.powi(count as i32);
        let mut qual_recalc :f32 = -TEN * (p_recalc).log(TEN)+33.0;
        // check for overflow before switching to u8
        if qual_recalc.is_infinite() || qual_recalc > 255.0 {
            qual_recalc = 255.0;
        }

        // switch to u8 and then the corresponding char
        let mut qual_recalc_char = qual_recalc.round() as u8 as char;
        if (qual_recalc_char as u8) > max_qual {
            qual_recalc_char = 'I';
        }
        // a reduction in quality is not expected... but just in case.
        if (qual_recalc_char as u8) < min_qual {
            qual_recalc_char = '!';
        }
        qual_out.push(qual_recalc_char);
    }

    return qual_out;
}
*/




