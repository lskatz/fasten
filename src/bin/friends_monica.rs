extern crate ross;
extern crate multiqueue;

use std::fs::File;
use std::io::BufReader;

use std::thread;

use ross::io::fastq;
use ross::io::seq::Seq;
use ross::io::seq::Cleanable;

fn main(){
    ross::parse_args();
    let numcpus :u64=12;

    let (tx, rx) = multiqueue::mpmc_queue(numcpus.pow(2));

    // receiving threads
    let mut handles = vec![];
    for _ in 0..numcpus {
        // Clone the receiver to sidestep variable ownership
        let cur_rx = rx.clone();
        handles.push(thread::spawn(move || {
            for seq_str in cur_rx {
                let mut seq = Seq::from_String(&seq_str);
                seq.thresholds.insert("min_length".to_string(),100.0);
                seq.thresholds.insert("min_avg_qual".to_string(),20.0);
                seq.thresholds.insert("min_trim_qual".to_string(),20.0);
                seq.lower_ambiguity_q();
                seq.trim();
                if seq.is_high_quality() {
                    seq.print();
                }
            }
        }));
    }
    rx.unsubscribe();

    // Read the file and send seqs to threads
    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut fastq_reader=fastq::Reader::new(my_buffer);
    while let Some(seq_obj) = fastq_reader.read_carefully() {
        let seq_str :String = seq_obj.to_string();
        tx.try_send(seq_str)
            .expect("Could not send seq_obj");
    }

    // Join the sender by dropping it.
    drop(tx);
    // Join the receiver threads.
    for t in handles {
        t.join()
            .expect("Could not join thread");
    }

}

