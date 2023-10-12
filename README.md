[![Crates.io](https://img.shields.io/badge/crates.io-v0.1-green.svg)](https://crates.io/crates/fasten)

# Fasten

Perform random operations on fastq files, using unix streaming.  Secure your analysis with Fasten!

## Synopsis

### read metrics
 
    $ cat testdata/R1.fastq testdata/R2.fastq | \
        fasten_shuffle | fasten_metrics | column -t
    totalLength  numReads  avgReadLength  avgQual
    800          8         100            19.53875

### read cleaning

    $ cat testdata/R1.fastq testdata/R2.fastq | \
        fasten_shuffle | \
        fasten_clean --paired-end --min-length 2 | \
        gzip -c > cleaned.shuffled.fastq.gz

    $ zcat cleaned.shuffled.fastq.gz | fasten_metrics | column -t
    totalLength  numReads  avgReadLength  avgQual
    800          8         100            19.53875
    # No reads were actually filtered with cleaning, with --min-length=2

_etc_

## Installation

Fasten is programmed in the Rust programming language.  More information about Rust, including installation and the executable `cargo`, can be found at [rust-lang.org](https://www.rust-lang.org).

After downloading, use the Rust executable `cargo` like so:

    cd fasten
    cargo build --release
    export PATH=$PATH:$(pwd)/target/release

All executables will be in the directory `fasten/target/release`.

## General usage

All scripts accept the parameters, read uncompressed fastq format from stdin, and print uncompressed fastq format to stdout.  All paired end fastq files must be in interleaved format, and they are written in [interleaved format](./docs/file-formats.md), except when deshuffling with `fasten_shuffle`.

* `--help`
* `--numcpus` Not all scripts will take advantage of numcpus. (not currently implemented)
* `--paired-end` Input reads are interleaved paired end
* `--verbose` Print more status messages

## Documentation

Please see the inline documentation at https://lskatz.github.io/fasten/fasten

This documentation was built with `cargo docs --no-deps`

## Other documentation

* Some workflows are shown in the [one-liners](./docs/one-liners.md) page.
* Some wrapper scripts are noted in the [scripts](./docs/scripts.md) page.

## Fasten script descriptions

|script             |Description|
|-------------------|-----------|
|[`fasten_clean`](https://lskatz.github.io/fasten/fasten_clean)     | Trims and cleans a fastq file.|
|[`fasten_convert`](https://lskatz.github.io/fasten/fasten_convert)   | Converts between different sequence formats like fastq, sam, fasta.|
|[`fasten_straighten`](https://lskatz.github.io/fasten/fasten_straighten)| Convert any fastq file to a standard four-line-per-entry format.|
|[`fasten_metrics`](https://lskatz.github.io/fasten/fasten_metrics)   | Prints basic read metrics.|
|[`fasten_pe`](https://lskatz.github.io/fasten/fasten_pe)        | Determines paired-endedness based on read IDs.|
|[`fasten_randomize`](https://lskatz.github.io/fasten/fasten_randomize) | Randomizes reads from input |
|[`fasten_combine`](https://lskatz.github.io/fasten/fasten_combine)   | Combines identical reads and updates quality scores.|
|[`fasten_kmer`](https://lskatz.github.io/fasten/fasten_kmer)      | Kmer counting.|
|[`fasten_normalize`](https://lskatz.github.io/fasten/fasten_normalize)      | Normalize read depth by using kmer counting.|
|[`fasten_sample`](https://lskatz.github.io/fasten/fasten_sample)    | Downsamples reads.|
|[`fasten_shuffle`](https://lskatz.github.io/fasten/fasten_shuffle)   | Shuffles or deshuffles paired end reads.|
|[`fasten_validate`](https://lskatz.github.io/fasten/fasten_validate)  | Validates your reads (deprecated in favor of `fasten_inspect` and `fasten_repair`|
|[`fasten_inspect`](https://lskatz.github.io/fasten/fasten_inspect)  | adds information to read IDs such as seqlength |
|[`fasten_repair`](https://lskatz.github.io/fasten/fasten_repair)  | Repairs corrupted reads |
|[`fasten_quality_filter`](https://lskatz.github.io/fasten/fasten_quality_filter) | Transforms nucleotides to "N" if the quality is low | |
|[`fasten_trim`](https://lskatz.github.io/fasten/fasten_trim)      | Blunt-end trims reads | |
|[`fasten_replace`](https://lskatz.github.io/fasten/fasten_replace)   | Find and replace using regex | |
|[`fasten_mutate`](https://lskatz.github.io/fasten/fasten_mutate)    | introduce random mutations | |
|[`fasten_regex`](https://lskatz.github.io/fasten/fasten_regex)     | Filter for reads using regex | |
|[`fasten_progress`](https://lskatz.github.io/fasten/fasten_progress)  | Add progress to any place in the pipeline | |
|[`fasten_sort`](https://lskatz.github.io/fasten/fasten_sort)  | Sort fastq entries | |

## Etymology

Many of these scripts have inspiration from the fastx toolkit, and I wanted to make a `fasty` which was already the name of a bioinformatics program.
Therefore I cycled through other letters of the alphabet and came across "N."  So it is possible to pronounce this project like "Fast-N" or in a way
that indicates that you are securing your analysis by "fasten"ing it (with a silent T).

## Acknowledgements

Thank you Henk Den Bakker for many helpful discussions around Rust, helping me name this software, and many other things.

