//! Sort a fastq file.
//! If the reads are paired end, then the sorted field 
//! concatenates R1 and R2 before comparisons in the sort.
//! R1 and R2 reads will stay together if paired end.
//! 
//! Sorting by GC content will give better compression by magic of gzip
//! and other algorithms.
//! 
//! Sorting can also aid in stable hashsums.
//! 
//! # Examples
//! 
//! ## stable hashsum
//! ```bash
//! cat file.fastq | fasten_sort | md5sum > file.fastq.md5
//! ```
//! ## better compression by sorting by GC content
//! ```bash
//! zcat file.fastq.gz | fasten_sort --sort-by GC | gzip -c > smaller.fastq.gz
//! 
//! ## get good compression from paired end reads
//! ```bash
//! zcat R1.fastq.gz R2.fastq.gz | fasten_shuffle | \
//!   fasten_sort --paired-end --sort-by GC | \
//!   fasten_shuffle -d -1 sorted_1.fastq -2 sorted_2.fastq && \
//!   gzip -v sorted_1.fastq sorted_2.fastq
//! ```
//! 
//! Compare compression between unsorted and sorted
//! from the previous example
//! 
//! ```bash
//! ls -lh sorted_1.fastq.gz sorted_2.fastq.gz
//! ```
//! 
//! ## Fast sorting of large files
//! 
//! If you want reads sorted but do not care if _everything_ is sorted,
//! you can sort in chunks. This is useful for streaming large files.
//! 
//! ```bash
//! zcat large.fastq.gz | fasten_sort --paired-end --chunk-size 1000 | gzip -c > sorted.fastq.gz
//! ```
//! 
//! # Usage 
//! 
//! ```text
//! Usage: fasten_sort [-h] [-n INT] [-p] [-v] [-s STRING] [-r]
//!
//! Options:
//!     -h, --help          Print this help menu.
//!     -n, --numcpus INT   Number of CPUs (default: 1)
//!     -p, --paired-end    The input reads are interleaved paired-end
//!     -v, --verbose       Print more status messages
//!     -s, --sort-by STRING
//!                         Sort by either SEQ, GC, or ID. If GC, then the entries
//!                         are sorted by GC percentage. SEQ and ID are
//!                         alphabetically sorted.
//!     -r, --reverse       Reverse sort
//!     -c, --chunk-size INT
//!                         If > 0, then chunks of reads or pairs will be sorted
//!                         instead of the whole set. This is useful for streaming
//!                         large files. Default: 0
//! ```

extern crate getopts;
extern crate fasten;
extern crate regex;
extern crate threadpool;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use fasten::fasten_base_options;
use fasten::fasten_base_options_matches;
//use fasten::logmsg;

#[test]
fn test_sort_fastq_basic () {
    let is_paired_end = false;
    let which_field   = "ID";
    let my_file = File::open("testdata/four_reads.fastq").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut buffer_iter = my_buffer.lines();

    let mut entries:Vec<Seq> = vec![];
    while let Some(line) = buffer_iter.next() {
        let id1  = line.expect("ERROR reading the ID line");
        let seq1 = buffer_iter.next().expect("ERROR reading a sequence line")
            .expect("ERROR reading a sequence line");
                  buffer_iter.next().expect("ERROR reading a plus line")
                      .expect("ERROR reading the plus line");
        let qual1= buffer_iter.next().expect("ERROR reading a qual line")
            .expect("ERROR reading a qual line");

        let mut id2   =String::new();
        let mut seq2  =String::new();
        let mut qual2 =String::new();

        // Get R2
        if is_paired_end {
            id2  = buffer_iter.next().expect("ERROR reading the ID line")
                .expect("ERROR reading the ID line");
            seq2 = buffer_iter.next().expect("ERROR reading a sequence line")
                .expect("ERROR reading a sequence line");
                       buffer_iter.next().expect("ERROR reading a plus line")
                          .expect("ERROR reading the plus line");
            qual2= buffer_iter.next().expect("ERROR reading a qual line")
                .expect("ERROR reading a qual line");
        }

        let entry:Seq = Seq {
            pe:     is_paired_end,
            id1:    id1,
            seq1:   seq1,
            qual1:  qual1,
            minimizer1: String::new(),
            id2:    id2,
            seq2:   seq2,
            qual2:  qual2,
            minimizer2: String::new()
        };

        entries.push(entry);

    }

    // test that each read is readX in a sorted order
    let sorted_entries:Vec<Seq> = sort_entries(entries.clone(), &which_field, false);
    for i in 0..4 {
        let expected = format!("@read{}",i);
        assert_eq!(sorted_entries[i].id1, expected, "Sort by ID");
    }

    // test reverse sorting
    let reverse_sorted_entries:Vec<Seq> = sort_entries(entries.clone(), &which_field, true);
    for i in 0..4 {
        let expected = format!("@read{}",i);
        let idx = 3 - i;
        assert_eq!(reverse_sorted_entries[idx].id1, expected, "Reverse sort by ID. Full list is {:?}", reverse_sorted_entries);
    }
}

/// A sequence struct that is paired-end aware
#[derive(Debug, Clone)]
struct Seq {
    pe:     bool,
    id1:    String,
    seq1:   String,
    qual1:  String,
    minimizer1: String,
    id2:    String,
    seq2:   String,
    qual2:  String,
    minimizer2: String,
}

fn main(){
    let mut opts = fasten_base_options();
    // Options specific to this script
    opts.optopt("s","sort-by","Sort by either SEQ, MINIMIZER, GC, or ID. If GC, then the entries are sorted by GC percentage. SEQ and ID are alphabetically sorted.","STRING");
    opts.optopt("k", "kmer-length", "Length of kmer if using minimizers", "STRING");
    opts.optflag("r","reverse","Reverse sort");
    opts.optopt("c", "chunk-size", "If > 0, then chunks of reads or pairs will be sorted instead of the whole set. This is useful for streaming large files. Default: 0", "INT");

    let matches = fasten_base_options_matches("Sort reads. This can be useful for many things including checksums and reducing gzip file sizes. Remember to use --paired-end if applicable.", opts);

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);

    let is_paired_end=matches.opt_present("paired-end");
    let reverse_sort =matches.opt_present("reverse");

    let which_field:String={
        if matches.opt_present("sort-by") {
            matches.opt_str("sort-by").expect("ERROR parsing --sort-by")
                .to_uppercase()
        } else {
            "ID".to_string()
        }
    };

    let _num_cpus:usize = {
        if matches.opt_present("numcpus") {
            matches.opt_str("numcpus").expect("ERROR parsing --numcpus")
                .parse()
                .expect("ERROR: numcpus is not an integer")
        } else {
            1
        }
    };

    let k: usize = matches.opt_get_default("kmer-length", 21).expect("ERROR parsing --kmer-length");
    let chunk_size: usize = matches.opt_get_default("chunk-size", 0).expect("ERROR parsing --chunk-size");

    let mut buffer_iter = my_buffer.lines();

    let mut entries:Vec<Seq> = vec![];
    while let Some(line) = buffer_iter.next() {
        let id1  = line.expect("ERROR reading the ID line");
        let seq1 = buffer_iter.next().expect("ERROR reading a sequence line")
            .expect("ERROR reading a sequence line");
                  buffer_iter.next().expect("ERROR reading a plus line")
                      .expect("ERROR reading the plus line");
        let qual1= buffer_iter.next().expect("ERROR reading a qual line")
            .expect("ERROR reading a qual line");

        let mut id2   =String::new();
        let mut seq2  =String::new();
        let mut qual2 =String::new();

        // Get R2
        if is_paired_end {
            id2  = buffer_iter.next().expect("ERROR reading the ID line")
                .expect("ERROR reading the ID line");
            seq2 = buffer_iter.next().expect("ERROR reading a sequence line")
                .expect("ERROR reading a sequence line");
                       buffer_iter.next().expect("ERROR reading a plus line")
                          .expect("ERROR reading the plus line");
            qual2= buffer_iter.next().expect("ERROR reading a qual line")
                .expect("ERROR reading a qual line");
        }

        // minimizers: they are costly if we aren't using them
        let mut minimizer1 = String::new();
        let mut minimizer2 = String::new();
        if which_field == "MINIMIZER" {
            minimizer1 = minimizer(&seq1, k);
            minimizer2 = minimizer(&seq2, k);
        }

        let entry:Seq = Seq {
            pe:     is_paired_end,
            id1:    id1, //format!("{} minimizer:{}",id1,minimizer1),
            seq1:   seq1,
            qual1:  qual1,
            minimizer1: minimizer1,
            id2:    id2,
            seq2:   seq2,
            qual2:  qual2,
            minimizer2: minimizer2
        };

        entries.push(entry);

        // If we are chunking, then sort and print the chunk
        if chunk_size > 0 && entries.len() == chunk_size {
            let sorted_entries:Vec<Seq> = sort_entries(entries, &which_field, reverse_sort);
            for entry in sorted_entries {
                println!("{}\n{}\n+\n{}", entry.id1, entry.seq1, entry.qual1);
                if entry.pe {
                    println!("{}\n{}\n+\n{}", entry.id2, entry.seq2, entry.qual2);
                }
            }
            entries = vec![];
        }

    }

    // If we aren't chunking then just print everything sorted
    if chunk_size == 0 {
        let sorted_entries:Vec<Seq> = sort_entries(entries, &which_field, reverse_sort);
        for entry in sorted_entries {
            println!("{}\n{}\n+\n{}", entry.id1, entry.seq1, entry.qual1);
            if entry.pe {
                println!("{}\n{}\n+\n{}", entry.id2, entry.seq2, entry.qual2);
            }
        }
    }

}

/// Find the lexicographically smallest kmer in a sequence.
fn minimizer (sequence:&str, k: usize) -> String {

    // if the length of the sequence is less than k, then return the sequence
    if sequence.len() < k {
        return sequence.to_string();
    }
    
    let mut min = "z".repeat(k);
    for i in 0..sequence.len()-k {
        let kmer = &sequence[i..i+k];
        if kmer < &min {
            min = kmer.to_string();
        }
    }

    return min;
}

/// Sort fastq entries in a vector
fn sort_entries (unsorted:Vec<Seq>, which_field:&str, reverse_sort:bool) -> Vec<Seq> {

    //logmsg(&format!("REVERSE SORT? {}\n{:?}",reverse_sort, &unsorted));

    // I want to avoid editing the vector in place
    let mut sorted = unsorted.clone();

    // Actual sort
    // TODO sort by length?
    match which_field{
        "ID" => {
            sorted.sort_by(|a, b| {
                if a.id1 != b.id1 {
                    a.id1.cmp(&b.id1)
                } else {
                    a.id2.cmp(&b.id2)
                }
            });
        },
        "SEQ" => {
            sorted.sort_by(|a, b| {
                if a.seq1 != b.seq1 {
                    a.seq1.cmp(&b.seq1)
                } else {
                    a.seq2.cmp(&b.seq2)
                }
            });
        },
        "GC"  => {
            sorted.sort_by(|a,b| {
                let a_seq = format!("{}{}", a.seq1, a.seq2);
                let b_seq = format!("{}{}", b.seq1, b.seq2);

                let a_gc:f32  = a_seq.matches(&['G','g','C','c']).count() as f32 / a_seq.len() as f32;
                let b_gc:f32  = b_seq.matches(&['G','g','C','c']).count() as f32 / b_seq.len() as f32;
                //logmsg(format!("{} ({}) <=> {} ({})", a.id1, a_gc, b.id1, b_gc));
                a_gc.partial_cmp(&b_gc).unwrap()
            });
        },
        "MINIMIZER" => {
            sorted.sort_by(|a,b| {
                // Sort by minimizer1 first
                if a.minimizer1 != b.minimizer1 {
                    a.minimizer1.cmp(&b.minimizer1)
                }
                // If the R1 minimizer isn't the same, then compare on minimizer2
                else if a.minimizer2 != b.minimizer2 {
                    a.minimizer2.cmp(&b.minimizer2)
                }
                // If both minimizers are the same, then sort by R1 SEQ
                else if a.seq1 != b.seq1 {
                    a.seq1.cmp(&b.seq1)
                }
                // and then lastely by R2 SEQ
                else {
                    a.seq2.cmp(&b.seq2)
                }
            });
        },
        _   => {
            panic!("Tried to sort by {} which is not implemented", which_field);
        }
    };

    if reverse_sort {
        //logmsg("REVERSE SORT");
        sorted.reverse();
    }

    return sorted;
}

