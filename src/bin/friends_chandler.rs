extern crate ross;
extern crate statistical;
extern crate getopts;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use ross::ross_base_options;
use ross::logmsg;

use std::collections::HashMap;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = ross_base_options();

    // script-specific options
    opts.optopt("k","kmer-length","The size of the kmer","INT");

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");

    if matches.opt_present("help") {
        println!("Counts kmers. Doesn't anyone remember that Chander is an analyst?\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }
    if matches.opt_present("paired-end") {
        logmsg("WARNING: --paired-end is not utilized in this script");
    }

    let kmer_length={
        if matches.opt_present("kmer-length") {
            matches.opt_str("kmer-length")
                .expect("ERROR: could not understand parameter --kmer-length")
                .parse()
                .expect("ERROR: --kmer-length is not an INT")
        } else {
            21
        }
    };


    let filename = "/dev/stdin";
    
    // read the file
    let my_file = File::open(&filename).expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut buffer_iter = my_buffer.lines();
    let mut kmer_hash :HashMap<String,u32> = HashMap::new();
    while let Some(_) = buffer_iter.next() {
        let seq = buffer_iter.next().expect("ERROR reading a sequence line")
            .expect("ERROR reading a sequence line");
        buffer_iter.next();
        buffer_iter.next();

        let seq_length=seq.len();
        // Don't count short sequences
        if seq_length < kmer_length {
            continue;
        }

        let my_num_kmers=seq_length - kmer_length + 1;
        for idx in 0..my_num_kmers {
            // increment the kmer count by reference
            let kmer_count = kmer_hash.entry(String::from(&seq[idx..kmer_length+idx])).
                or_insert(0);
            *kmer_count += 1;
        }

        // kmer count on the revcomp sequence too
        let revcomp = revcomp(&seq);
        for idx in 0..my_num_kmers {
            let kmer_count = kmer_hash.entry(String::from(&revcomp[idx..kmer_length+idx]))
                .or_insert(0);
            *kmer_count += 1;
        }
    }


    // TODO in the future: somehow efficiently remove reverse
    // complement kmers before printing because it is basically
    // double the information needed.
    for (kmer,count) in kmer_hash.iter() {
        println!("{}\t{}",kmer,count);
    }
}


/// reverse-complement a dna sequence
// Thanks Henk for supplying these functions.
fn revcomp(dna: &str) -> String {
    let mut rc_dna: String = String::with_capacity(dna.len());
    for c in dna.chars().rev() {
        rc_dna.push(switch_base(c))
    }
    rc_dna
}

/// Complementary nucleotide
fn switch_base(c: char) -> char {
    match c {
        'a' => 't',
        'c' => 'g',
        't' => 'a',
        'g' => 'c',
        'u' => 'a',
        'n' => 'n',
        'A' => 'T',
        'C' => 'G',
        'T' => 'A',
        'G' => 'C',
        'U' => 'A',
        'N' => 'N',
        _ => 'N',
    }
}

