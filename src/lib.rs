extern crate regex;
extern crate statistical;
extern crate getopts;
use std::env;
use std::path::Path;

use getopts::Options;

pub mod io;

/// a function that reads an options object and adds ROSS default options.
pub fn ross_base_options() -> Options{
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help menu.");
    opts.optopt("n","numcpus","Number of CPUs (default: 1)","INT");
    opts.optflag("p","paired-end","The input reads are interleaved paired-end");

    return opts;
}

/// Print a formatted message to stderr 
pub fn logmsg(msg: &str) {
    let args: Vec<_> = env::args().collect();
    // is there a better way to get the basename of the program???
    let program = Path::file_name(Path::new(&args[0])).unwrap().to_str().unwrap();
    eprintln!("{}: {}", &program, &msg);
}
