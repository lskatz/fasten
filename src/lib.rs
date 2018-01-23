extern crate regex;
extern crate statistical;
extern crate getopts;

use std::env;
use getopts::Options;

pub mod io;

pub fn parse_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help menu.");
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

