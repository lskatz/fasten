extern crate getopts;
extern crate ross;
extern crate rand;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use rand::Rng;

use ross::ross_base_options;


fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = ross_base_options();
    //script-specific flags
    opts.optopt("f","frequency","The frequency of reads to print","FLOAT");

    let matches = opts.parse(&args[1..]).expect("Error: could not parse parameters");
    if matches.opt_present("help") {
        println!("Create random reads from stdin. Phoebe is totally random!\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let is_pe = matches.opt_present("paired-end");

    let frequency={
        if matches.opt_present("frequency") {
            matches.opt_str("frequency")
                .expect("Error parsing --frequency")
                .parse()
                .expect("ERROR: --frequency is not a float")
        } else {
            1.0
        }
    };

    print_reads_from_stdin(is_pe, frequency);
}

fn print_reads_from_stdin(is_pe: bool, frequency: f32) -> () {
    // Random number generator
    let mut rng = rand::thread_rng();

    let my_file = File::open("/dev/stdin").expect("Could not open stdin");
    let my_buffer=BufReader::new(my_file);
    let mut line_iterator = my_buffer.lines();
    while let Some(id) = line_iterator.next() {

        // Get all 8 lines. I am hoping that there is a
        // speedup if I don't loop over the line iterator
        // seven times.
        let mut entry = id.expect("ERROR parsing ID line");
        entry.push('\n');
        entry.push_str(&line_iterator
                       .next()
                       .expect("ERROR getting sequence line")
                       .expect("ERROR parsing sequence line")
                       );
        entry.push('\n');
        entry.push_str(&line_iterator
                       .next()
                       .expect("ERROR getting plus line")
                       .expect("ERROR parsing plus line")
                       );
        entry.push('\n');
        entry.push_str(&line_iterator
                       .next()
                       .expect("ERROR getting qual line")
                       .expect("ERROR parsing qual line")
                       );
        entry.push('\n');

        if is_pe {
            entry.push_str(&line_iterator
                           .next()
                           .expect("ERROR getting id line")
                           .expect("ERROR parsing id line")
                           );
            entry.push('\n');
            entry.push_str(&line_iterator
                           .next()
                           .expect("ERROR getting sequence line")
                           .expect("ERROR parsing sequence line")
                           );
            entry.push('\n');
            entry.push_str(&line_iterator
                           .next()
                           .expect("ERROR getting plus line")
                           .expect("ERROR parsing plus line")
                           );
            entry.push('\n');
            entry.push_str(&line_iterator
                           .next()
                           .expect("ERROR getting qual line")
                           .expect("ERROR parsing qual line")
                           );
            entry.push('\n');
        }
        let m :f32 = rng.gen_range(0.0, 1.0);

        if m < frequency {
            println!("{}", entry.trim());
        }
    }
}

