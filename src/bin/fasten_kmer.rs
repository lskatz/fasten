//! Counts kmers.
//! Each line is a kmer with two columns separated by tab: kmer, count
//! Optional columns starting with column 3 are the reads that start with that kmer
//! with a delimiter of `~`

//! # Examples

//! ```bash
//! cat testdata/four_reads.fastq | fasten_kmer -k 15 > 15mers.tsv
//! ```

//! # Usage

//! ```text
//! Usage: fasten_kmer [-h] [-n INT] [-p] [-v] [-k INT]

//! Options:
//!    -h, --help          Print this help menu.
//!    -n, --numcpus INT   Number of CPUs (default: 1)
//!    -p, --paired-end    The input reads are interleaved paired-end
//!    -v, --verbose       Print more status messages
//!    -k, --kmer-length INT
//!                        The size of the kmer
//! ```
    
extern crate fasten;
extern crate statistical;
extern crate getopts;

use std::io::BufReader;
use std::io::BufRead;
use std::io::stdin;
use std::io::Stdin;

use fasten::fasten_base_options;
use fasten::fasten_base_options_matches;
use fasten::logmsg;

use std::collections::HashMap;

/// Glues together paired end reads internally and is a
/// character not expected in any read
const READ_SEPARATOR :char = '~';

#[test]
/// Let's count some kmers on homopolymers
fn test_kmer_counting_homopolymers () {
    let k = 4;

    // The first easy test is an AAAA kmer in a Ax10 homopolymer
    let a_homopolymer = (0..10).map(|_| "A").collect::<String>();
    let mut kmer = kmers_in_str(&a_homopolymer, k, false);
    let obs_a_count = *kmer.entry(String::from("AAAA")).or_insert(0);
    assert_eq!(obs_a_count, 7, "10-mer A yields seven 4-mers of A");
    let obs_t_count = *kmer.entry(String::from("TTTT")).or_insert(0);
    assert_eq!(obs_t_count, 0, "10-mer A yields zero 4-mers of T");

    // Ok but what about revcom kmers
    let mut kmer = kmers_in_str(&a_homopolymer, k, true );
    let obs_a_count = *kmer.entry(String::from("AAAA")).or_insert(0);
    assert_eq!(obs_a_count, 7, "10-mer A yields seven 4-mers of A");
    let obs_t_count = *kmer.entry(String::from("TTTT")).or_insert(0);
    assert_eq!(obs_t_count, 7, "10-mer A yields seven 4-mers of T");
}   
#[test]
/// Let's count some kmers on something more complicated
fn test_kmer_counting_4mers () {
    let k = 4;

    // This is the first seq of the test file four_reads.fastq
    let seq = "AAAGTGCTCTTAACTTGTCCCGCTCCACATCAGCGCGACATCAATCGACATTAAACCGAGTATCTTGTCAGCCTGGGGTGACGATGCGTCCCATTAAAGT";
    let my_kmer = "TTAA";
    let mut kmer = kmers_in_str(&seq, k, false);
    let obs = *kmer.entry(String::from(my_kmer)).or_insert(0);
    assert_eq!(obs, 3, "Found {} kmer in seq {} times. Seq is \n{}", my_kmer, obs, seq);

    // But also test for revcom kmers
    let mut kmer = kmers_in_str(&seq, k, true );
    let obs = *kmer.entry(String::from(my_kmer)).or_insert(0);
    assert_eq!(obs, 6, "Found {} kmer in seq {} times. Seq is \n{}", my_kmer, obs, seq);

    // one more kmer
    let my_kmer = "TGTC";
    let mut kmer = kmers_in_str(&seq, k, false);
    let obs = *kmer.entry(String::from(my_kmer)).or_insert(0);
    assert_eq!(obs, 2, "Found {} kmer in seq {} times. Seq is \n{}", my_kmer, obs, seq);
    // and revcom
    let mut kmer = kmers_in_str(&seq, k, true );
    let obs = *kmer.entry(String::from(my_kmer)).or_insert(0);
    assert_eq!(obs, 4, "Found {} kmer in seq {} times. Seq is \n{}", my_kmer, obs, seq);
}

fn main(){
    let mut opts = fasten_base_options();

    // script-specific options
    let default_k:usize = 21;
    opts.optopt("k","kmer-length",&format!("The size of the kmer (default: {})",default_k),"INT");
    opts.optflag("r","revcomp", "Count kmers on the reverse complement strand too");
    opts.optflag("m","remember-reads", "Add reads to subsequent columns. Each read begins with the kmer. Only lists reads in the forward direction.");

    let matches = fasten_base_options_matches("Counts kmers.", opts);
    
    //if matches.opt_present("paired-end") {
    //    logmsg("WARNING: --paired-end is not utilized in this script");
    //}

    let kmer_length:usize={
        if matches.opt_present("kmer-length") {
            matches.opt_str("kmer-length")
                .expect("ERROR: could not understand parameter --kmer-length")
                .parse()
                .expect("ERROR: --kmer-length is not an INT")
        } else {
            default_k
        }
    };


    let stdin = stdin();
    
    count_kmers(stdin, kmer_length, matches.opt_present("revcomp"), matches.opt_present("remember-reads"), matches.opt_present("paired-end"));
}

/// Read fastq from stdin and count kmers
fn count_kmers (stdin:Stdin, kmer_length:usize, revcomp:bool, remember_reads:bool, paired_end:bool) {

    // keep track of which sequences start with which kmers
    let mut kmer_to_seqs :HashMap<String, Vec<String>> = HashMap::new();
    
    // read the file
    let my_buffer=BufReader::new(stdin);
    let mut buffer_iter = my_buffer.lines();
    let mut kmer_hash :HashMap<String,u32> = HashMap::new();
    while let Some(id_opt) = buffer_iter.next() {
        let seq = buffer_iter.next().expect("ERROR reading a sequence line")
            .expect("ERROR reading a sequence line");
        // burn the plus line
        let plus_opt = buffer_iter.next();
        // burn the qual line
        let qual_opt = buffer_iter.next();

        // get all the kmers in this entry
        let entry_kmers = kmers_in_str(&seq, kmer_length, revcomp);
        // merge the entry kmers
        for (key, value) in entry_kmers.iter() { 
            let kmer_count = kmer_hash.entry(String::from(key)).
                or_insert(0);
            *kmer_count += value;
        }

        // If this is paired end and if we're saving the second pair's
        // read, then reserve a declaired variable here for the string.
        let mut r2_read_string :String = String::new();
        if paired_end {
            let id2 = buffer_iter.next().expect("reading the ID2 line")
                .expect("reading the ID2 line");

            let seq2 = buffer_iter.next().expect("ERROR reading a sequence line, second in pair")
                .expect("ERROR reading a sequence line, second in pair");
            // burn the plus line
            let plus2 = buffer_iter.next().expect("reading the plus2 line")
                .expect("reading the plus2 line");
            // burn the qual line
            let qual2 = buffer_iter.next().expect("reading the qual2 line")
                .expect("reading the qual2 line");

            // get all the kmers in this entry
            let entry_kmers2 = kmers_in_str(&seq2, kmer_length, revcomp);
            // merge the entry kmers
            for (key, value) in entry_kmers2.iter() { 
                let kmer_count = kmer_hash.entry(String::from(key)).
                    or_insert(0);
                *kmer_count += value;
            }

            r2_read_string = vec![id2,seq2,plus2,qual2].join(&READ_SEPARATOR.to_string());
        }

        // Remember the read that initiated this
        if remember_reads {
            let init_kmer = String::from(&seq[0..kmer_length]);
            let init_kmer_vec = kmer_to_seqs.entry(init_kmer).or_insert(vec![]);

            // get the formatted entry
            let id   = id_opt.expect("reading the ID line");
            let plus = plus_opt.expect("reading the plus line")
                .expect("reading the plus line");
            let qual = qual_opt.expect("reading the qual line")
                .expect("reading the qual line");

            if paired_end {
                init_kmer_vec.push(
                    vec![id, seq, plus, qual, r2_read_string].join(&READ_SEPARATOR.to_string())
                );
            }
            else {
                init_kmer_vec.push(
                    vec![id, seq, plus, qual].join(&READ_SEPARATOR.to_string())
                );
            }
        }
    }


    // TODO in the future: somehow efficiently remove reverse
    // complement kmers before printing because it is basically
    // double the information needed.
    for (kmer,count) in kmer_hash.iter() {
        let mut line :String = format!("{}\t{}", kmer, count);
        if remember_reads {
            let reads_vec = kmer_to_seqs.entry(kmer.to_string()).or_insert(vec![]);
            for read in reads_vec {
                line.push_str("\t");
                line.push_str(read);
            }
        }

        println!("{}", line);
    }
}

/// Read a str of nucleotides and count kmers.
/// If `should_revcomp` is true, then will also count kmers on the opposite strand.
fn kmers_in_str (seq:&str, kmer_length:usize, should_revcomp:bool) -> HashMap<String,u32> {
    // save the kmers in this local hash
    let mut kmer_hash :HashMap<String,u32> = HashMap::new();
    // how many kmers we expect in this sliding window
    let seq_len = seq.len();
    // Don't count short sequences
    if seq_len < kmer_length {
        logmsg("WARNING: found a sequence less than k");
        return kmer_hash;
    }
    let my_num_kmers = seq_len - kmer_length + 1;

    for idx in 0..my_num_kmers {
        // increment the kmer count by reference
        let kmer_count = kmer_hash.entry(String::from(&seq[idx..kmer_length+idx])).
            or_insert(0);
        *kmer_count += 1;
    }

    // kmer count on the revcomp sequence too
    if should_revcomp {
        let revcomp = revcomp(&seq);
        for idx in 0..my_num_kmers {
            let kmer_count = kmer_hash.entry(String::from(&revcomp[idx..kmer_length+idx]))
                .or_insert(0);
            *kmer_count += 1;
        }
    }

    return kmer_hash;
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

/// Complementary nucleotide for ACTGUN, case insensitive
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

