//! Repairs reads from fasten_inspect output
//! 
//! # Examples
//! ```bash
//! ./target/debug/fasten_inspect  < testdata/four_reads.fastq | ./target/debug/fasten_repair
//!
//! ```
//!
//! If remove-info is given, then extra header information from fasten_inspect will be removed.
//!
//! # Usage
//!
//! ```text
//! Usage: fasten_repair [-h] [-n INT] [-p] [--verbose] [--version] [--min-length INT] [--min-quality FLOAT] [--remove-info] [-m STRING]
//! Options:
//!    -h, --help          Print this help menu.
//!    -n, --numcpus INT   Number of CPUs (default: 1)
//!    -p, --paired-end    The input reads are interleaved paired-end
//!        --verbose       Print more status messages
//!        --version       Print the version of Fasten and exit
//!        --min-length INT
//!                        Minimum read length allowed
//!        --min-quality FLOAT
//!                        Minimum quality allowed
//!        --remove-info   Remove fasten_inspect headers
//!    -m, --mode STRING   Either repair or panic. If panic, then the binary will
//!                        panic when the first issue comes up. Default:repair
//! ```
//!
//! # Methods of repair
//!
//! If you choose `--mode repair`, then this is the expected behavior
//!
//! * Mismatched seq and qual lengths: seq or qual length will be truncated
//!
//! # Panic
//!
//! If the sequences are not repaired but there is still an issue, the program might still panic:
//!
//! * seq length < min length (TODO when implementing PE reads)
//! * avg qual < min qual (TODO when implementing PE reads)
//! * invalid characters in seq (TODO when implementing PE reads)
//! * invalid characters in qual (TODO when implementing PE reads)
//! * `@` not present in first character of the entry (TODO when implementing PE reads)
//! * `+` not present in the first character of the third line (TODO when implementing PE reads)
//! 

extern crate getopts;
extern crate fasten;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

use fasten::fasten_base_options;
use fasten::fasten_base_options_matches;
use fasten::logmsg;

fn main(){
    let mut opts = fasten_base_options();
    // Options specific to this script
    opts.optopt("","min-length","Minimum read length allowed","INT");
    opts.optopt("","min-quality","Minimum quality allowed","FLOAT");
    opts.optflag("", "remove-info", "Remove fasten_inspect headers");
    opts.optopt("m", "mode", " Either repair or panic. If panic, then the binary will panic when the first issue comes up. Default:repair", "STRING");

    let matches = fasten_base_options_matches("Repairs reads", opts);

    let paired_end = matches.opt_present("paired-end");

    let min_length :usize={
        if matches.opt_present("min-length") {
            matches.opt_str("min-length")
                .expect("ERROR parsing min-length")
                .parse()
                .expect("ERROR parsing min-length as INT")
        } else {
            0
        }
    };
    let min_qual :f32={
        if matches.opt_present("min-quality") {
            matches.opt_str("min-quality")
                .expect("ERROR parsing min-quality")
                .parse()
                .expect("ERROR parsing min-quality as FLOAT")
        } else {
            0.0
        }
    };

    let remove_info :bool = matches.opt_present("remove-info");
    let mode :String = {
        if matches.opt_present("mode") {
            matches.opt_str("mode")
                .expect("ERROR parsing mode")
        } else {
            "repair".to_string()
        }
    };

    repair_reads(paired_end, min_length, min_qual, remove_info, &mode);
}

fn repair_reads(paired_end:bool, min_length: usize, min_qual: f32, remove_info: bool, mode: &str) {
    //behavior
    let should_repair :bool = {
        if mode == "repair" {
            true
        } else {
            false
        }
    };

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let mut my_buffer = BufReader::new(my_file);

    let mut id   = String::new();
    let mut seq  = String::new();
    let mut plus = String::new();
    let mut qual = String::new();

    //let mut i = 0;
    loop{

        id.clear();
        if my_buffer.read_line(&mut id).expect("Cannot read new line") == 0 {
            break;
        }
        let r1_id = id.clone();
        
        seq.clear();
        my_buffer.read_line(&mut seq).expect("ERROR: failed to read 'seq' line");
        plus.clear();
        my_buffer.read_line(&mut plus).expect("ERROR: failed to read 'plus' line");
        qual.clear();
        my_buffer.read_line(&mut qual).expect("ERROR: failed to read 'qual' line");
        id   = id.trim().to_string();
        seq  = seq.trim().to_string();
        plus = plus.trim().to_string();
        qual = qual.trim().to_string();

        let (r1, is_r1_good, err1):(String, bool, String) = repair_one_read(id.clone(), seq.clone(), plus.clone(), qual.clone(), should_repair, min_length, min_qual, remove_info);
        //i += 4;

        let mut is_r2_good = true;
        let mut r2 = "".to_string();
        let mut err2 = "".to_string();
        let mut r2_id :String = "".to_string();
        if paired_end {
            id.clear();
            if my_buffer.read_line(&mut id).expect("Cannot read new line") == 0 {
                panic!("ERROR: paired end expected but not found after R1 {}", r1_id);
            }
            r2_id = id.clone();
            
            seq.clear();
            my_buffer.read_line(&mut seq).expect("ERROR: failed to read 'seq' line");
            plus.clear();
            my_buffer.read_line(&mut plus).expect("ERROR: failed to read 'plus' line");
            qual.clear();
            my_buffer.read_line(&mut qual).expect("ERROR: failed to read 'qual' line");
            id   = id.trim().to_string();
            seq  = seq.trim().to_string();
            plus = plus.trim().to_string();
            qual = qual.trim().to_string();

            (r2, is_r2_good, err2) = repair_one_read(id.clone(), seq.clone(), plus.clone(), qual.clone(), should_repair, min_length, min_qual, remove_info);
        }

        if is_r1_good && is_r2_good{
            // Start with R1 for printing.
            let mut to_print = r1.clone();
            // If we're looking at a paired end, then add R2 to R1 for printing.
            if paired_end {
                to_print.push_str("\n");
                to_print.push_str(&r2);
            } 
            println!("{}", to_print);

            // If R1 and R2 are good, then any "errors" are warnings.
            // Print the warnings.
            if err1.as_str() != "" {
                logmsg(format!("WARNING(s) on R1: {}", err1));
            }
            if err2.as_str() != "" {
                logmsg(format!("WARNING(s) on R2: {}", err2));
            }
        } else {
            
            // Print the errors.
            if err1.as_str() != "" {
                logmsg(format!("SKIP R1 {}\n=> {}\n", r1_id.trim(), err1));
            }
            if err2.as_str() != "" {
                logmsg(format!("SKIP R2 {}\n=> {}\n", r2_id.trim(), err2));
            }
        }
    }
}

/// Repair exactly one read
fn repair_one_read(mut id:String, mut seq:String, plus:String, mut qual:String, should_repair:bool, min_length: usize, min_qual: f32, remove_info: bool) -> (String, bool, String) {
    // Eventual error message if any
    let mut error = String::new();
    let mut num_errors = 0;

    // The eventual sequence identifier with fasten_inspect info or not
    let mut identifier = String::new();
    // Information about the read from the defline
    let mut f:HashMap<&str, &str> = HashMap::new();
    // Get that information from the defline
    for field in id.split_whitespace() {
        match field.find(":") {
            None => {
                identifier.push_str(&field);
                continue;
            },
            Some(_) => {},
        };
        let mut key_value = field.split(':');
        let key   :&str = key_value.next().expect("key not found");
        let value :&str = key_value.next().expect("value not found");
        f.insert(key, value);
    }

    // get some variables out of the hash
    let seq_length  :usize = f.entry("seq-length").or_insert("0").parse::<usize>().unwrap();
    let qual_length :usize = f.entry("qual-length").or_insert("0").parse::<usize>().unwrap();
    let avg_qual    :f32   = f.entry("avg-qual").or_insert("0").parse::<f32>().unwrap();
    let seq_invalid_chars :&str  = f.entry("seq-invalid-chars").or_insert("");
    let qual_invalid_chars :&str = f.entry("qual-invalid-chars").or_insert("");
    let _read_pair :u8 = f.entry("read-pair").or_insert("1").parse::<u8>().unwrap(); // either 1 or 2
    // these are either 1 (true) or 0 (false)
    let id_at :u8 = f.entry("id-at").or_insert("0").parse::<u8>().unwrap();
    let id_plus :u8 = f.entry("id-plus").or_insert("0").parse::<u8>().unwrap();

    // Check seq length and qual length
    if seq_length != qual_length {
        if should_repair {
            let new_length :usize = *vec![seq_length, qual_length].iter().min().unwrap();
            seq  = seq[..new_length].to_string();
            qual = qual[..new_length].to_string();
            error.push_str(
                &format!("Repaired sequence and qual length\n")
            );
            // Don't count this as an actual error and so don't increment.
        } else {
            panic!("ERROR: seq length({}) did not match qual length({}) on seqid {}\n", &seq_length, &qual_length, &id);
        }
    }
    if seq_length < min_length {
        error.push_str(
            &format!("seq length({}) is less than min length specified ({})\n", &seq_length, &min_length)
        );
        num_errors += 1;
    }
    // Check quality score
    if avg_qual < min_qual {
        error.push_str(
            &format!("average quality ({}) is less than min quality ({})\n", &avg_qual, &min_qual)
        );
        num_errors += 1;
    }

    // check key seq-invalid-chars
    if seq_invalid_chars != "" {
        error.push_str(
            &format!("invalid seq characters found in {}\n", &id)
        );
        num_errors += 1;
    }
    // check key qual-invalid-chars
    if qual_invalid_chars != "" {
        error.push_str(
            &format!("invalid qual characters found in {}\n", &id)
        );
        num_errors += 1;
    }

    // check key id-at
    if id_at < 1 {
        error.push_str(
            &format!("no @ found at position 1 on line 1 for {}\n", &id)
        );    
        num_errors += 1;
    }
    // check key id-plus 
    if id_plus < 1 {
        error.push_str(
            &format!("no + found at position 1 on line 3 for {}\n", &id)
        );
        num_errors += 1;
    }
            
    // if the user requests, we can remove all fasten_inspect information
    if remove_info {
        id = identifier.clone();
    }
    
    let entry :String = format!("{}\n{}\n{}\n{}", &id, &seq, &plus, &qual);

    let mut is_good = true;
    if num_errors > 0 {
        is_good = false;
    }
    return (entry, is_good, error);
}

