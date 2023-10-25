---
title: 'Fasten with Pipes'
tags:
  - command line
  - fastq manipulation
  - interleaved fastq
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

There are still many gaps for basic bioinformatics on the command line for standard file formats.
Bioinformaticians have been able to use many tools to manipulate sequence data files in the fastq format, such as `seqkit`, `seqtk` or FASTX-Toolkit.
These tools only accept paired end (PE) sequence data when split into multiple files per sample.
However, some bioinformaticians prefer to combine PE data from a single sample into one file using the interleaved fastq file format, but this format is not always well supported in mainstream tools.
Here, we provide Fasten to the community to address these needs.

# Materials

We leveraged the Cargo packaging system in Rust to create a basic framework for interleaved fastq file manipulation.
Each executable reads from `stdin` and prints reads to `stdout` and only performs one function at a time.
The core executables perform these fundamental functions: 1) converting to and from interleaved format, 2) converting to and from other sequence file formats, 3) ‘straightening’ fastq files to a more standard 4-line-per-entry format.

There are 20 executables including but not limited to read metrics, read cleaning, kmer counting, read validation, and regular expressions for interleaved fastq files.

We have also taken advantage of Rust to make comprehensive and standardized documentation.
Continuous integration was implemented in GitHub Actions for unit testing, containerizing, and benchmarking.
Benchmarking was performed against other mainstream packages using `hyperfine` using 100 replicates and 2 burn-ins.

# Results

Documentation, the container, and code are available at GitHub. Benchmarking results were graphed into \autoref{fig:benchmarks}.

[!Six benchmarks. From left to right, then to bottom: Searching for a sequence in a fastq file with either `seqkit grep` or `fasten_regex`; downsampling reads using either `fasten sample` or `seqtk sample`; interleaving reads from R1 and R2 files not using `fasten_progress`, using it before shuffle, or using it after shuffle; sorting fastq entries by sequence with `fasten_sort` or `seqkit sort`; sorting sequences by id; and converting nonstandard fastq files to a format whose entries are four lines each.\label{fig:benchmarks}](benchmarks.png)

# Conclusions

Fasten is a powerful manipulation suite for interleaved fastq files, written in Rust.
We benchmarked Fasten on several categories.
It has strengths as shown in Figure 1 but it does not occupy the fastest position in all cases.
We also tested whether `fasten_progress` slowed computation in certain situations.
We conclude that creating a progress meter after a blocking operation such as `fasten_shuffle` maintains the balance between speed and observing the progress.

Fasten touts a comprehensive manual, continuous integration, and integration into the command line with unix pipes.
It is well poised to be a crucial module for daily work on the command line.

# Acknowledgements

Thank you John Phan for creating the Docker container.

