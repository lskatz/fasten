[![Build Status](https://travis-ci.org/lskatz/fasten.svg?branch=master)](https://travis-ci.org/lskatz/fasten)

# Fasten

Perform random operations on fastq files, using unix streaming.  Secure your analysis with Fasten!

## Installation

Fasten is programmed in the Rust programming language.  More information about Rust, including installation and the executable `cargo`, can be found at [rust-lang.org](https://www.rust-lang.org).

After downloading, use the Rust executable `cargo` like so:

    cd fasten
    cargo build --release

All executables will be in the directory `fasten/target/release`.

## General usage

All scripts accept the parameters, read uncompressed fastq format from stdin, and print uncompressed fastq format to stdout.  All paired end fastq files must be in interleaved format, and they are written in [interleaved format](./docs/file-formats.md), except when deshuffling with `fasten_shuffle`.

* `--help`
* `--numcpus` Not all scripts will take advantage of numcpus.
* `--paired-end` Input reads are interleaved paired end
* `--verbose` Print more status messages

## Other documentation

* Some workflows are shown in the [one-liners](./docs/one-liners.md) page.
* Some wrapper scripts are noted in the [scripts](./docs/scripts.md) page.

## Fasten script descriptions

|script             |Description|
|-------------------|-----------|
|`fasten_clean`     | Trims and cleans a fastq file. She likes to clean.|
|`fasten_straighten`| Convert any fastq file to a standard four-line-per-entry format.|
|`fasten_metrics`   | Prints basic read metrics.|
|`fasten_pe`        | Determines paired-endedness based on read IDs.|
|`fasten_randomize` | Randomizes reads from input |
|`fasten_combine`   | Combines identical reads and updates quality scores.|
|`fasten_kmer`      | Kmer counting.|
|`fasten_sample`    | Downsamples reads.|
|`fasten_shuffle`   | Shuffles or deshuffles paired end reads.|
|`fasten_validate`  | Validates your reads|
|`fasten_quality_filter` | Transforms nucleotides to "N" if the quality is low | |
|`fasten_trim`      | Blunt-end trims reads | |
|`fasten_replace`   | Find and replace using regex | |
|`fasten_regex`     | Filter for reads using regex | |


