extern crate regex;
extern crate statistical;
extern crate getopts;

use std::env;
use getopts::Options;

pub mod io;

/// Parsing arguments in ROSS code.
/// Each element in the vector is a vector of &str.
pub fn parse_args(additional_opts: &Vec<Vec<&str>>) -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    // Mandatory flags.
    opts.optflag("h", "help", "Print this help menu.");
    // Add in additional options.
    for opt in additional_opts {
        opts.optflag(opt[0], &opt[1], &opt[2]);
    }

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => {m}
        Err(error) => { panic!(error.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(opts);
        std::process::exit(1);
    }

    args
}

pub fn print_usage(opts: Options) {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

