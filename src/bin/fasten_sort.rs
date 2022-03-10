extern crate getopts;
extern crate fasten;
extern crate regex;
extern crate threadpool;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use fasten::fasten_base_options;
use fasten::logmsg;

#[derive(Debug, Clone)]
struct Seq {
    pe:     bool,
    id1:    String,
    seq1:   String,
    qual1:  String,
    id2:    String,
    seq2:   String,
    qual2:  String,
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = fasten_base_options();
    // Options specific to this script
    opts.optopt("s","sort-by","Sort by either SEQ or ID","STRING");

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");
    if matches.opt_present("help") {
        println!("Sort reads.\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);

    let is_paired_end=matches.opt_present("paired-end");

    let which_field:String={
        if matches.opt_present("sort-by") {
            matches.opt_str("sort-by").expect("ERROR parsing --sort-by")
                .to_uppercase()
        } else {
            "SEQ".to_string()
        }
    };

    let num_cpus:usize = {
        if matches.opt_present("numcpus") {
            matches.opt_str("numcpus").expect("ERROR parsing --numcpus")
                .parse()
                .expect("ERROR: numcpus is not an integer")
        } else {
            1
        }
    };

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
            id2:    id2,
            seq2:   seq2,
            qual2:  qual2
        };

        entries.push(entry);

    }

    // TODO read the parameter for which field to sort by
    logmsg("Note: sorting by ID");
    entries.sort_by(|b, a| {
        let a_id = format!("{}{}", a.id1, a.id2);
        let b_id = format!("{}{}", b.id1, b.id2);
        a_id.cmp(&b_id)
    });

    for entry in entries {
        println!("{}\n{}\n+\n{}", entry.id1, entry.seq1, entry.qual1);
    }
}

