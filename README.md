# Random Operations on Sequences Suite - ROSS

Perform random operations on fastq files, using unix streaming.  This is an implementation of the ROSS project but in the Rust language.

## Installation

After downloading, use the Rust executable `cargo` like so:

    cd ROSS.rs
    cargo build --release

All executables will be in the directory `ROSS.rs/target/release`.

## General usage

All scripts accept the parameters

* `--help`
* `--numcpus` Not all scripts will take advantage of numcpus.

## Ross script descriptions

*Not all scripts have been created at this time.*

|script               |Description|    |
|---------------------|-----------|----|
|`friends_monica`  | Trims and cleans a fastq file| ![Monica](/images/monica.jpg) |
|`friends_carol`   | Convert any fastq file to a standard four-line-per-entry format| ![Carol](/images/carol.jpg) | 
|`friends_rachel`  | Prints basic read metrics| ![Rachel](/images/rachel.jpg) |
|`friends_ung`     | Determines paired-endedness| ![UNG](/images/UNG.png) |
|`friends_ross`    | Makes sure a fastq file is in a standard format and is unbroken | ![Ross](/images/ross.png) | 
|`friends_phoebe`  | Randomizes reads| ![Phoebe](/images/phoebe.png) |
|`friends_chandler`| Pure perl kmer counting. No outside dependencies.| ![Chandler](/images/chander.png) |
|`friends_marcel`  | Rescores reads based on kmer abundance | ![Marcel](/images/marcel.png) | 
|`friends_ursula`  | Removes duplicate reads and/or downsamples reads| ![Ursula](/images/ursula.png) | 
|`friends_joey`    | Shuffles or deshuffles paired end reads| ![Joey](/images/joey.png) |
|`friends_barry`   | Joins overlapping paired ends together | ![Barry](/images/barry.png) |
|`friends_gunther` | Validates your reads ... and *you* | ! [Gunther](/images/gunther.png) |

