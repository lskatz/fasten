//! Perform random operations on fastq files, using unix streaming.
//! Secure your analysis with Fasten!
//! # Synopsis
//! ## read metrics
//! ```text
//! 
//! $ cat testdata/R1.fastq testdata/R2.fastq | \
//!     fasten_shuffle | fasten_metrics | column -t
//! totalLength  numReads  avgReadLength  avgQual
//! 800          8         100            19.53875
//! ```
//! 
//! ## read cleaning
//!
//! ```text
//! $ cat testdata/R1.fastq testdata/R2.fastq | \
//!     fasten_shuffle | \
//!     fasten_clean --paired-end --min-length 2 | \
//!     gzip -c > cleaned.shuffled.fastq.gz
//! 
//! $ zcat cleaned.shuffled.fastq.gz | fasten_metrics | column -t
//! totalLength  numReads  avgReadLength  avgQual
//! 800          8         100            19.53875
//! ```
//! _NOTE_: No reads were actually filtered with cleaning, with --min-length=2
//!
//! ## Kmer counting
//! ```text
//! $ cat testdata/R1.fastq | \
//!   fasten_kmer -k 21 > 21mers.tsv
//! ```
//! 
//! ## Read sampling
//! ```text
//! $ cat testdata/R1.fastq testdata/R2.fastq | \
//!     fasten_shuffle | \
//!     fasten_sample --paired-end --frequency 0.1 > 10percent.fastq
//! ```
//!
//! # Advanced
//! ## Set of downsampled reads
//! Create a set of downsampled reads for a titration experiment
//! and clean them
//! ```text
//! for frequency in 0.1 0.2 0.3 0.4 0.5; do
//!   cat testdata/R1.fastq testdata/R2.fastq | \
//!     fasten_shuffle | \
//!     fasten_clean --paired-end --min-length 50 --min-trim-quality 25
//!     fasten_sample --paired-end --frequency $frequency > cleaned.$frequency.fastq
//! done
//! ```
//!
//! ## Validate a whole directory of fastq reads
//! ```text
//! \ls *_1.fastq.gz | xargs -n 1 -P 4 bash -c '
//!   echo -n "." >&2 # progress bar
//!   R1=$0
//!   R2=${0/_1.fastq.gz/_2.fastq.gz}
//!   zcat $R1 $R2 | fasten_shuffle | fasten_validate --paired-end
//! '
//! ```

extern crate regex;
extern crate statistical;
extern crate getopts;
use std::env;
use std::path::Path;

use getopts::Options;
use getopts::Matches;

/// input/output methods
pub mod io;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Have some strings that can be printed which could be
/// used to propagate errors between piped scripts.

/// Invalid fastq ID (no @)
static INVALID_ID  :&'static str= "invalid_id";
/// Invalid sequence (underscore)
static INVALID_SEQ :&'static str= "invalid_seq";
/// Invalid plus line (no +)
static INVALID_PLUS:&'static str= "invalid_plus";
/// Invalid qual line (~ is chr 126 when the normal max number is 40)
static INVALID_QUAL:&'static str= "invalid_qual";

/// Propagate an error by printing invalid read(s)
pub fn eexit() -> () {
    println!("{}\n{}\n{}\n{}",INVALID_ID,INVALID_SEQ,INVALID_PLUS,INVALID_QUAL);
    std::process::exit(1);
}

/// Rewrite print!() so that it doesn't panic on broken
/// pipe.
#[macro_export]
macro_rules! print (
    // The extra scope is necessary so we don't leak imports
    ($($arg:tt)*) => ({
        // The `write!()` macro is written so it can use `std::io::Write`
        // or `std::fmt::Write`, this import sets which to use
        use std::io::{self, Write};
        if let Err(_) = write!(io::stdout(), $($arg)*) {
            // Optionally write the error to stderr
            ::std::process::exit(0);
        }
        
    })
);

/// a function that reads an options object and adds fasten default options.
pub fn fasten_base_options() -> Options{
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help menu.");
    opts.optopt("n","numcpus","Number of CPUs (default: 1)","INT");
    opts.optflag("p","paired-end","The input reads are interleaved paired-end");
    opts.optflag("v","verbose","Print more status messages");
    opts.optflag("v","version","Print the version of Fasten and exit");

    return opts;
}

/// a function that processes the options on the command line
pub fn fasten_base_options_matches(opts:Options) -> Matches{
    let args: Vec<String> = env::args().collect();
    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");

    if matches.opt_present("version") {
        println!("Fasten v{}", &VERSION);
        std::process::exit(0);
    }
    if matches.opt_present("h") {
        println!("{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    return matches
}

/// Print a formatted message to stderr 
pub fn logmsg<S: AsRef<str>>(stringlike: S) {
    let args: Vec<_> = env::args().collect();
    // is there a better way to get the basename of the program???
    let program = Path::file_name(Path::new(&args[0])).unwrap().to_str().unwrap();
    let str_ref = stringlike.as_ref();
    eprintln!("{}: {}", &program, str_ref);
}

