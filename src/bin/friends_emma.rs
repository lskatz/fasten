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
const READ_SEPARATOR :char = '~';

fn main(){
    let args: Vec<String> = env::args().collect();
    let opts = ross_base_options();

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");

    if matches.opt_present("h") {
        println!("Emma: collapse identical reads into single reads, recalculating quality values. If paired end, then each set of reads must be identical to be collapsed. Rachel's daughter Emma was played by twins, essentially collapsing two individuals into one character!");
        println!("{}",opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let paired_end = matches.opt_present("paired-end");

    // seq => {seq2 => count}
    let mut seq_count :HashMap<String, u32>   =HashMap::new();
    // seq => {seq2 => vec![sequence of prob of errors]}
    let mut seq_error_rate :HashMap<String, Vec<f32>> = HashMap::new();

    /*
    let mut current_seq1 = String::new();
    let mut current_seq2 = String::new();
    //let mut current_key  = String::new();
    let mut current_qual1= String::new();
    let mut current_qual2= String::new();
    //let mut combined_qual= String::new();
    */

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut lines = my_buffer.lines();
    while let Some(_) = lines.next() {
        // TODO transform current_seq1 to its minimum
        let current_seq1 = lines.next()
            .expect("ERROR getting seq line")
            .expect("ERROR parsing seq line");
        lines.next()
            .expect("ERROR getting plus line")
            .expect("ERROR parsing plus line");
        let current_qual1= lines.next()
            .expect("ERROR getting qual line")
            .expect("ERROR parsing qual line");
        let mut seq_key = current_seq1.clone();
        let mut combined_qual = current_qual1.clone();
        if paired_end {
            lines.next()
                .expect("ERROR getting id line")
                .expect("ERROR parsing id line");
            // TODO transform current_seq2 to its minimum
            let current_seq2 = lines.next()
                .expect("ERROR getting seq line")
                .expect("ERROR parsing seq line");
            lines.next()
                .expect("ERROR getting plus line")
                .expect("ERROR parsing plus line");
            let current_qual2= lines.next()
                .expect("ERROR getting qual line")
                .expect("ERROR parsing qual line");
            seq_key.push(READ_SEPARATOR); 
            seq_key.push_str(&current_seq2);
            combined_qual.push(READ_SEPARATOR);
            combined_qual.push_str(&current_qual2);
        }
        let count = seq_count.entry(seq_key.clone()).or_insert(0);
        *count += 1;

        // If this sequence hasn't been seen yet,
        // then instantiate the probabilities.
        if !seq_error_rate.contains_key(&seq_key) {
            let mut qual_vec :Vec<f32> = Vec::new();
            for qual_char in combined_qual.chars() {
                let qual_int = qual_char as u8 as f32 - 33.0;
                let p :f32 = TEN.powf((-1.0 * qual_int)/TEN);
                qual_vec.push(p);
            }
            seq_error_rate.insert(seq_key.clone(), qual_vec);
        }
        // If this sequence has been seen yet, then
        // start combining the error rates.
        else {
            let qual_vec = seq_error_rate.entry(seq_key.clone()).or_insert(Vec::new());

            let these_errors = combined_qual.chars().map(|qual_char|{
                let qual_int = qual_char as u8 as f32 - 33.0;
                let p :f32 = TEN.powf((-1.0 * qual_int)/TEN);
                return p;
            }).collect();
            let new_qual = combine_error_vectors(&qual_vec,&these_errors);
            *qual_vec = new_qual;
        }
    }

    let max_qual_char = 'I';
    let min_qual_char = '!';
    let max_qual = max_qual_char as u8;
    let min_qual = min_qual_char as u8;

    let mut seq_counter=0;
    for (seq,combined_qual) in seq_error_rate {
        seq_counter += 1;

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





