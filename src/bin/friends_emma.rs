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
const READ_SEPARATOR :&'static str = "~~~~";
const READ_SEPARATOR_LENGTH :usize = 4;

fn main(){
    let args: Vec<String> = env::args().collect();
    let opts = ross_base_options();

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");

    if matches.opt_present("h") {
        println!("Emma: collapse identical reads into single reads, recalculating quality values. If paired end, then each set of reads must be identical. Rachel's daughter Emma was played by twins, essentially collapsing two individuals into one character!");
        println!("{}",opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let lines_per_read={
        if matches.opt_present("paired-end") {
            8
        }else{
            4
        }
    };

    // Keep track of IDs
    // id => seq
    // id => qual
    let mut seq_id    :HashMap<String,String>=HashMap::new();
    //let mut qual_id   :HashMap<String,String>=HashMap::new();
    // ID => count
    let mut seq_count :HashMap<String,u32>   =HashMap::new();
    // keep track of sequence pairing
    // ID1 => ID2
    let mut seq_pair  :HashMap<String,String>=HashMap::new();
    // ID => vec![quality cigar String]
    let mut qual      :HashMap<String,Vec<char>>=HashMap::new();

    // SEQ1~~~~SEQ2 => Vec<ID1~~~~ID2>
    let mut identical_seq :HashMap<String,Vec<String>>=HashMap::new();

    let mut current_id1  = String::new();
    let mut current_id2  = String::new();
    let mut current_seq1 = String::new();
    let mut current_seq2 = String::new();

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut line_counter =0;
    for line in my_buffer.lines() {
        match line_counter % lines_per_read {
            0 => {
                current_id1 = line.expect("Could not unwrap ID1");
            }
            4 => {
                current_id2 = line.expect("Could not unwrap ID2");
            }
            1 => {
                current_seq1 = line.expect("Could not unwrap seq1");
                let count = seq_count.entry(current_id1.clone()).or_insert(0);
                *count += 1;

                seq_id.insert(current_id1.clone(),current_seq1.clone());
            }
            5 => {
                let current_seq2 = line.expect("Could not unwrap seq2");
                let count = seq_count.entry(current_id2.clone()).or_insert(0);
                *count += 1;

                seq_id.insert(current_id2.clone(),current_seq2.clone());

                seq_pair.insert(current_id1.clone(),current_id2.clone());
            }

            // 3 and 7: the qual lines
            3 => {
                let qual1 = line.expect("Could not unwrap qual1");
                qual.insert(current_id1.clone(),qual1.chars().collect());
                if lines_per_read == 4 {
                    let mut seq_set :&Vec<String> = identical_seq.entry(current_seq1.clone()).or_insert(Vec::new());
                    seq_set.push(current_id1.clone());
                }

            }
            7 => {
                let qual2 = line.expect("Could not unwrap qual2");
                qual.insert(current_id2.clone(),qual2.chars().collect());
            }
            _=>{}
        }
        line_counter += 1;
    }

    // collapse the reads
    for (id1,id2) in seq_pair {
        println!("{} => {}",id1,id2);
    }
}

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





