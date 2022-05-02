---
title: 'Fasten with Pipes'
tags:
  - command line
  - fastq
  - one more keyword
authors:
  - name: Lee S. Katz
    affiliation: "1, 2"
    orcid: 0000-0002-2533-9161
  - name: Henk C. den Bakker
    orcid: 0000-0002-4086-1580
    affiliation: 1
affiliations:
 - name: Enteric Diseases Laboratory Branch (EDLB), Centers for Disease Control and Prevention, Atlanta, GA, USA
   index: 1
 - name: Center for Food Safety, University of Georgia, Griffin, GA, USA
   index: 2
bibliography: paper.bib
---

# Background

There are still many gaps for basic bioinformatics on the command line for basic file formats.
Historically for fastq files, bioinformaticians have been able to use many different tools such as `seqkit`, `seqtk` or FASTX-Toolkit.
When considering one genome sample and a set of paired end reads, it can be confusing or overwhelming to have two files per sample.
Therefore, some bioinformaticians have used the interleaved fastq file format.
However, it is not well supported in mainstream tools.
Here, we provide Fasten to the community to address these needs.

# Materials

We leveraged the Cargo packaging system in Rust to create a basic framework for interleaved fastq file manipulation.
Each executable reads from `stdin` and prints reads to `stdout` and only performs one function at a time.
The core executables perform these fundamental functions:
1) converting to and from interleaved format, 
2) converting to and from other sequence file formats,
3) 'straightening' fastq files to a more standard 4-line-per-entry format.

There are 18 executables including but not limited to read metrics, read cleaning, kmer counting, read validation, and regular expressions for interleaved fastq files.

We have also taken advantage of Rust to make comprehensive documentation, available at lskatz.github.io/lskatz/fasten/fasten.

Continuous integration was implemented in GitHub Actions for unit testing, containerizing, and benchmarking.

# Results

Documentation, the container, and code are available at github.com/lskatz/fasten.

Benchmarking yielded times available at https://github.com/lskatz/fasten/actions/workflows/benchmark.yml

# Conclusions

Fasten is a powerful manipulation suite for interleaved fastq files, written in Rust.
It touts a comprehensive manual, continuous integration, and integration into the command line with unix pipes.

