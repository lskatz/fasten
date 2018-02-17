extern crate regex;
extern crate statistical;
extern crate getopts;
use std::env;
use std::path::Path;

pub mod io;


// TODO a function that reads an options object and adds
// default options.

/// Print a formatted message to stderr 
pub fn logmsg(msg: &str) {
    let args: Vec<_> = env::args().collect();
    // is there a better way to get the basename of the program???
    let program = Path::file_name(Path::new(&args[0])).unwrap().to_str().unwrap();
    eprintln!("{}: {}", &program, &msg);
}
