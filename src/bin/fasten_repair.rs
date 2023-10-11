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

    let mut i = 0;
    loop{

        id.clear();
        if my_buffer.read_line(&mut id).expect("Cannot read new line") == 0 {
            break;
        }

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

        let mut identifier = String::new();
        let mut f:HashMap<&str, &str> = HashMap::new();
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
                eprintln!("Repaired sequence length for {}", &id);
            } else {
                panic!("ERROR: seq length({}) did not match qual length({}) on seqid {}", &seq_length, &qual_length, &id);
            }
        }
        if seq_length < min_length {
            // TODO skip this sequence
            //   TODO consider whether this is paired-end
            panic!("ERROR: seq length({}) is less than min length specified ({})", &seq_length, &min_length);
        }
        // Check quality score
        if avg_qual < min_qual {
            panic!("ERROR: average quality ({}) is less than min quality ({})", &avg_qual, &min_qual);
        }

        // check key seq-invalid-chars
        if seq_invalid_chars != "" {
            panic!("ERROR: invalid seq characters found in {}", &id);
        }
        // check key qual-invalid-chars
        if qual_invalid_chars != "" {
            panic!("ERROR: invalid qual characters found in {}", &id);
        }

        // check key id-at
        if id_at < 1 {
            panic!("ERROR: no @ found at position 1 on line 1 for {}", &id);
        }
        // check key id-plus 
        if id_plus < 1 {
            panic!("ERROR: no + found at position 1 on line 3 for {}", &id);
        }

        i += 4;

        let mut denominator = 4;
        if paired_end {
            denominator = 8;
        }
        if (i % denominator) != 0 {
            panic!("ERROR: incomplete lines in this latest entry with ID {}", &id);
        }
        
        // TODO check that read1/read2 alternate
        
        // if the user requests, we can remove all fasten_inspect information
        if remove_info {
            id = identifier.clone();
        }
        println!("{}\n{}\n{}\n{}", &id, &seq, &plus, &qual);
    }

    // TODO if paired end, make sure we end on second read

}

