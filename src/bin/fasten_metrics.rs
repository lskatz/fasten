extern crate fasten;
extern crate statistical;
extern crate getopts;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::f32;

use std::env;

use fasten::fasten_base_options;
use fasten::logmsg;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = fasten_base_options();

    // script-specific options
    opts.optflag("","each-read","Print the metrics for each read. This creates very large output");
    opts.optopt("","distribution","Print the distribution for each metric. Must supply either 'normal' or 'nonparametric'","STRING");

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");

    if matches.opt_present("help") {
        println!("Gives read metrics on a read set.\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }
    if matches.opt_present("paired-end") {
        logmsg("WARNING: --paired-end is not utilized in this script");
    }

    let each_read :bool=matches.opt_present("each-read");

    let distribution = if matches.opt_present("distribution") {
        matches.opt_str("distribution")
            .expect("ERROR: could not understand parameter for --distribution")
    } else {
        String::new()
    };

    let filename = "/dev/stdin";
    
    // Header
    if each_read {
        println!("readID\treadLength\tavgQual");
    } else {
        println!("{}",vec!["totalLength", "numReads", "avgReadLength","avgQual"].join("\t"));

    }
    
    let mut read_length :Vec<f32> = vec![];
    let mut read_qual   :Vec<f32> = vec![];
    let mut num_lines   :u32   =0;

    // read the file
    let my_file = File::open(&filename).expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    for line in my_buffer.lines() {
        num_lines+=1;

        match num_lines % 4 {
            1 => {
                if each_read {
                    let id = line.expect("Expected an ID line");
                    print!("{}\t",&id[1..]);
                }
            }
            2 => {
                let my_read_length=line.expect("Expected a sequence line").len() as f32;
                if each_read {
                    print!("{}\t",my_read_length);
                }
                read_length.push(my_read_length);
            }
            0 => {
                let mut my_read_qual :u32=0;
                let qual_line=line.expect("Expected a qual line");
                for qual_char in qual_line.chars() {
                    my_read_qual += qual_char as u8 as u32 - 33;
                }
                if each_read {
                    println!("{}",my_read_qual as f32/qual_line.len() as f32);
                }
                read_qual.push(my_read_qual as f32 / qual_line.len() as f32);
            }
            _ => {

            }
        };
    }
    let num_reads :u32 = num_lines / 4;
    let total_length = read_length.iter().fold(0.0,|a,&b| a+b);

    let mut summary_metrics=vec![total_length.to_string(),num_reads.to_string()];

    // add statistics if requested
    let mut total_length_str = (total_length as f32/num_reads as f32).to_string();
    let mut total_qual_str   = ((read_qual.iter().fold(0.0,|a,&b| a+b)) / num_reads as f32).to_string();
    if distribution == "normal" {
        total_length_str.push_str("±");
        total_length_str.push_str(&standard_deviation(&read_length).to_string());
        total_qual_str.push_str("±");
        total_qual_str.push_str(&standard_deviation(&read_qual).to_string());

        summary_metrics.push(total_length_str);
        summary_metrics.push(total_qual_str);
    } else if distribution == "nonparametric" {
        eprintln!("WARNING: nonparametric distribution not yet supported");

    } else if distribution == "" {
        summary_metrics.push(total_length_str);
        summary_metrics.push(total_qual_str);
    } else {
        panic!("I did not understand --distribution {}",distribution);
    }

    // summary metrics
    if !each_read {
        println!("{}", summary_metrics.join("\t"));
    }

}

fn standard_deviation(vec :&Vec<f32>) -> f32{

    let num_data_points = vec.len();
    let avg :f32 = vec.iter().fold(0.0,|a,&b| a+b) as f32 / num_data_points as f32;

    let mut sum_squares :f32 = 0.0;
    for int in vec {
        sum_squares += (*int as f32 - avg).powi(2);
    }

    let avg_square_diff = sum_squares / (num_data_points - 1) as f32;
    
    return avg_square_diff.sqrt();

}
