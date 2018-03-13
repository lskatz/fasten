# Random Operations on Sequences Suite - ROSS

Perform random operations on fastq files, using unix streaming.  This is an implementation of the ROSS project but in the Rust language.

## Installation

After downloading, use the Rust executable `cargo` like so:

    cd ROSS.rs
    cargo build --release

All executables will be in the directory `ROSS.rs/target/release`.

## General usage

All scripts accept the parameters, read uncompressed fastq format, and print uncompressed fastq format to stdout.

* `--help`
* `--numcpus` Not all scripts will take advantage of numcpus.

## Ross script descriptions

*Not all scripts have been created at this time.*

|script               |Description|    |
|---------------------|-----------|----|
|`friends_monica`  | Trims and cleans a fastq file. She likes to clean.| ![Monica](/images/monica.jpg) |
|`friends_carol`   | Convert any fastq file to a standard four-line-per-entry format. Carol can help straighten you out.| ![Carol](/images/carol.jpg) | 
|`friends_rachel`  | Prints basic read metrics. Rachel tells you how good you look.| ![Rachel](/images/rachel.jpg) |
|`friends_ung`     | Determines paired-endedness. Ugly naked guy has gone through a lot of scrutiny.| ![UNG](/images/UNG.png) |
|`friends_phoebe`  | Randomizes reads. Phoebe is like, totally random.| ![Phoebe](/images/phoebe.png) |
|`friends_chandler`| Pure perl kmer counting. No outside dependencies. Chandler's job is rarely discussed but he does... I want to say, something in accounting?| ![Chandler](/images/chander.png) |
|`friends_marcel`  | Rescores reads based on kmer abundance. Marcel monkeys around with your data. | ![Marcel](/images/marcel.png) | 
|`friends_ursula`  | Removes duplicate reads and/or downsamples reads. Ursula is a twin but played by the same actress!| ![Ursula](/images/ursula.png) | 
|`friends_joey`    | Shuffles or deshuffles paired end reads. Joey can socialize!| ![Joey](/images/joey.png) |
|`friends_barry`   | Joins overlapping paired ends together. They might not always like it, but Barry can be a connection between friends of friends. | ![Barry](/images/barry.png) |
|`friends_gunther` | Validates your reads ... and *you* | ![Gunther](/images/gunther.png) |

[![Build Status](https://travis-ci.org/lskatz/ROSS.rs.svg?branch=master)](https://travis-ci.org/lskatz/ROSS.rs)

