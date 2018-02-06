extern crate getopts;
extern crate ross;
extern crate multiqueue;

use std::fs::File;
use std::io::BufReader;

use std::thread;

use ross::io::fastq;
use ross::io::seq::Seq;
use ross::io::seq::Cleanable;

use std::env;
use getopts::Options;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    // ROSS flags.
    // TODO put these options into ROSS to streamline.
    opts.optflag("h", "help", "Print this help menu.");
    opts.optopt("n","numcpus","Number of CPUs (default: 1)","INT");
    // Options specific to this script
    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");

    if matches.opt_present("h") {
        println!("{}", opts.usage(&opts.short_usage(&args[0])));
    }

    // defaults
    let mut numcpus :u32=1;
    if matches.opt_present("numcpus") {
        numcpus = matches.opt_str("numcpus")
            .expect("ERROR: could not read the numcpus argument")
            .parse()
            .expect("ERROR: numcpus is not an int");
    }
    if numcpus < 2 {
        numcpus=2;
    }

    //let capacity = numcpus.pow(4) as u64;
    let (tx, rx) = multiqueue::mpmc_queue(10000);

    // receiving threads
    let mut handles = vec![];
    // Take away one cpu for the controller thread.
    for _ in 0..numcpus-1 {
        // Clone the receiver to sidestep variable ownership
        let cur_rx = rx.clone();
        handles.push(thread::spawn(move || {
            for seq_buffer in cur_rx {
                for seq_str in seq_buffer {
                    let mut seq = Seq::from_string(&seq_str);
                    if seq.qual.len() == 0 {
                        eprintln!("Terminating signal!");
                        return;
                    }
                    seq.thresholds.insert("min_length".to_string(),100.0);
                    seq.thresholds.insert("min_avg_qual".to_string(),20.0);
                    seq.thresholds.insert("min_trim_qual".to_string(),20.0);
                    seq.lower_ambiguity_q();
                    seq.trim();
                    if seq.is_high_quality() {
                       seq.print();
                    }
                }
            }
        }));
    }
    rx.unsubscribe();

    // Read the file and send seqs to threads
    let mut seq_buffer = vec![];
    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let fastq_reader=fastq::FastqReader::new(my_buffer);
    let mut i = 0;
    for seq_obj in fastq_reader {
        if seq_obj.seq.len() == 0 {
            continue;
        }
        let seq_str :String = seq_obj.to_string();
        seq_buffer.push(seq_str);
        i+=1;
        if i % 100000 == 0 {
            eprintln!("Sent {} reads",i);
            let send_buffer = seq_buffer.clone();
            tx.try_send(send_buffer)
                .expect("Could not send the sequence buffer");
            seq_buffer = vec![];
        }
    }
    // One last send. Also include some blanks to tell the
    // threads to terminate.
    for _ in 0..numcpus {
      seq_buffer.push(Seq::blank().to_string());
    }
    tx.try_send(seq_buffer)
        .expect("Could not send the rest of the buffer");
    eprintln!("Sent {} reads in total.",i);

    // Join the sender by dropping it.
    drop(tx);
    // Join the receiver threads.
    for t in handles {
        t.join()
            .expect("Could not join thread");
    }

}

