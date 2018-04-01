extern crate ross;
extern crate getopts;
extern crate rand;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

use std::env;
use std::f32;

use ross::ross_base_options;
//use ross::logmsg;

// need this constant because the compiler had a problem
// with the syntax 10.0.pow()
const TEN: f32 = 10.0;
//const READ_SEPARATOR :&'static str = "~~~~";
//const READ_SEPARATOR_LENGTH :usize = 4;

fn main(){
    let args: Vec<String> = env::args().collect();
    let opts = ross_base_options();

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");

    if matches.opt_present("h") {
        println!("Emma: collapse identical reads into single reads, recalculating quality values. If paired end, then each set of reads must be identical. Rachel's daughter Emma was played by twins, essentially collapsing two individuals into one character!");
        println!("{}",opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let lines_per_read=4;

    // seq => {seq2 => count}
    let mut seq_count :HashMap<String, u32>   =HashMap::new();
    // seq => {seq2 => vec![sequence of prob of errors]}
    let mut seq_error_rate :HashMap<String, Vec<f32>> = HashMap::new();

    let mut current_seq1 = String::new();
    //let mut current_seq2 = String::new();
    //let mut current_qual1= String::new();
    //let mut current_qual2= String::new();

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut line_counter =0;
    for line in my_buffer.lines() {
        match line_counter % lines_per_read {
            1 => {
                current_seq1 = line.expect("Could not unwrap seq1");
                // TODO transform current_seq1 to its minimum
                let count = seq_count.entry(current_seq1.clone()).or_insert(0);
                *count += 1;
            }

            // 3 and 7: the qual lines
            3 => {
                let qual1_string = line.expect("Could not unwrap qual1");
                // If this sequence hasn't been seen yet,
                // then instantiate the probabilities.
                if !seq_error_rate.contains_key(&current_seq1) {
                    let mut qual1_vec :Vec<f32> = Vec::new();
                    for qual_char in qual1_string.chars() {
                        let qual_int = qual_char as u8 as f32 - 33.0;
                        let p :f32 = TEN.powf((-1.0 * qual_int)/TEN);
                        qual1_vec.push(p);
                    }
                    seq_error_rate.insert(current_seq1.clone(), qual1_vec);
                } 
                // If this sequence has been seen yet, then
                // start combining the error rates.
                else {
                    let qual1_vec = seq_error_rate.entry(current_seq1.clone()).or_insert(Vec::new());

                    let these_errors = qual1_string.chars().map(|qual_char|{
                        let qual_int = qual_char as u8 as f32 - 33.0;
                        let p :f32 = TEN.powf((-1.0 * qual_int)/TEN);
                        return p;
                    }).collect();
                    let new_qual = combine_error_vectors(&qual1_vec,&these_errors);
                    *qual1_vec = new_qual;
                }
            }
            _=>{}
        }
        line_counter += 1;
    }

    let max_qual = 'I' as u8;
    let min_qual = '#' as u8;

    let mut seq_counter=0;
    for (seq,combined_qual) in seq_error_rate {
        seq_counter += 1;

        // Make a new cigar line for quality
        let mut qual_cigar = String::new();
        for p in combined_qual {
            let mut qual_recalc :f32 = -TEN * (p).log(TEN)+33.0;
            // check for overflow before switching to u8
            if qual_recalc.is_infinite() || qual_recalc > 255.0 {
                qual_recalc = 255.0;
            }

            // switch to u8 and then the corresponding char
            let mut qual_recalc_char = qual_recalc.floor() as u8 as char;
            if (qual_recalc_char as u8) > max_qual {
                qual_recalc_char = 'I';
            }
            // a reduction in quality is not expected... but just in case.
            if (qual_recalc_char as u8) < min_qual {
                qual_recalc_char = '#';
            }
            qual_cigar.push(qual_recalc_char);
        }

        println!("@{}\n{}\n+\n{}",seq_counter,seq,qual_cigar);
    }
}


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

#[allow(dead_code)]
fn recalculate_qual(qual_str: &str, count: u32) -> String {
    let mut qual_out = String::new();

    let max_qual = 'I' as u8;
    let min_qual = '#' as u8;

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
        let mut qual_recalc_char = qual_recalc.floor() as u8 as char;
        if (qual_recalc_char as u8) > max_qual {
            qual_recalc_char = 'I';
        }
        // a reduction in quality is not expected... but just in case.
        if (qual_recalc_char as u8) < min_qual {
            qual_recalc_char = '#';
        }
        qual_out.push(qual_recalc_char);
    }

    return qual_out;
}





