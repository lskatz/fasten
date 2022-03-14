//! Prints a progress meter for number of fastq entries to stderr.
//! 
//! # Examples
//! 
//! ## fasten_metrics progress
//! While getting read metrics for a large fastq file, print the progress
//!  to make the wait a little easier
//! ```bash
//! cat large.fastq | fasten_progress | fasten_metrics
//! ```
//! ## fasten_shuffle progress
//! While shuffling a large fastq file, print the progress
//! ```bash
//! cat large_1.fastq large_2.fastq | fasten_progress | fasten_shuffle > interleaved.fastq
//! ```
//! 
//! # Usage
//! 
//! ```text
//! Usage: fasten_progress [-h] [-n INT] [-p] [-v] [--id STRING] [--update-every INT]
//! 
//! Options:
//!     -h, --help          Print this help menu.
//!     -n, --numcpus INT   Number of CPUs (default: 1)
//!     -p, --paired-end    The input reads are interleaved paired-end
//!     -v, --verbose       Print more status messages
//!         --id STRING     Progress identifier. Default: unnamed
//!         --update-every INT
//!                         Update progress every n reads.
//! ```

extern crate getopts;
extern crate fasten;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use fasten::fasten_base_options;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = fasten_base_options();
    // Options specific to this script
    opts.optopt("","id","Progress identifier. Default: unnamed","STRING");
    opts.optopt("","update-every","Update progress every n reads.","INT");

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");
    if matches.opt_present("help") {
        println!("Prints a progress meter for number of fastq entries.\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let progress_id:String = {
      if matches.opt_present("id") {
        matches.opt_str("id")
          .expect("ERROR parsing --id")
      } else {
        String::from("unnamed")
      }
    };

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);

    let lines_per_read :usize = 4;
    let update_every :usize  = {
        if matches.opt_present("update-every") {
            let tmp :usize = 
                matches.opt_str("update-every")
                 .expect("ERROR parsing update-every")
                 .parse()
                 .expect("ERROR parsing update-every as INT");
            tmp * lines_per_read
            //tmp
        } else {
            100
        }
    };

    let mut line_counter = 0;
    eprint!("\r{} progress: {}", progress_id, line_counter/lines_per_read);
    for _line in my_buffer.lines() {
        //let _line=line.expect("ERROR: did not get a line");
        line_counter += 1;

        // TODO pattern match for each kind of line:
        // id, seq, +, qual
        match line_counter % update_every {
            0=>{
                //println!("UPDATE: {}", line_counter);
                eprint!("\r{} progress: {}", progress_id, line_counter/lines_per_read);
                //eprint!(".");
            }
            _=>{

            }
        }
    }
    eprint!("\n");

    let msg = format!("{}: Finished progress on {} reads", progress_id, line_counter);
    fasten::logmsg(&msg);
}


