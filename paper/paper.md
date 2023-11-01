---
title: 'Fasten: a toolkit for streaming operations on fastq files'
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
 - name: Enteric Diseases Laboratory Branch (EDLB), Centers for Disease Control and Prevention, Atlanta, GA, United States of America
   index: 1
 - name: Center for Food Safety, University of Georgia, Griffin, GA, United States of America
   index: 2
bibliography: paper.bib
---

## Statement of need

There are still many gaps in basic command line tools for the handling of standard file formats in the field of bioinformatics.
Bioinformaticians have been able to use many tools to manipulate sequence data files in the fastq format, such as `seqkit` [@seqkit], `seqtk` [@seqtk] or FASTX-Toolkit [@fastx].
These tools only accept paired end (PE) sequence data when split into multiple files per sample.
Additionally, these tools do not always allow for Unix-style pipe file control. Sometimes they require explicity input/output options instead of using `stdin` and `stdout`.
However, some bioinformaticians prefer to combine PE data from a single sample into one file using the interleaved fastq file format, but this format is not always well supported in mainstream tools.
Here, we provide Fasten to the community to address these needs.

## Materials

We leveraged the Cargo packaging system in Rust to create a basic framework for interleaved fastq file manipulation.
Each executable reads from `stdin` and prints reads to `stdout` and only performs one function at a time.
The core executables perform these fundamental functions: 1) converting to and from interleaved format, 2) converting to and from other sequence file formats, 3) ‘straightening’ fastq files to a more standard 4-line-per-entry format.

There are 20 executables including but not limited to read metric generation, read cleaning, kmer counting, read validation, and regular expressions for interleaved fastq files.

We have also taken advantage of Rust to make comprehensive and standardized documentation.
Continuous integration was implemented in GitHub Actions for unit testing, containerizing, and benchmarking.
Benchmarking was performed against other mainstream packages using `hyperfine` using 20 replicates and 2 burn-ins [@Peter_hyperfine_2023].

## Results

Documentation, the container, and code are available at GitHub. Benchmarking results were graphed into Figures \label{fig:clean} - \label{fig:straighten}.

![Trimming with a minimum quality score \label{fig:clean}](../tests/hyperfine/clean.json.png)

![converting fastq to fasta \label{fig:convertToFasta}](../tests/hyperfine/convertToFasta.json.png)

![interleaving R1 and R2 reads \label{fig:interleave}](../tests/hyperfine/interleave.json.png)

![kmer counting \label{fig:kmer}](../tests/hyperfine/kmer.json.png)

![normalizing read depth using kmer coverage \label{fig:normalize}](../tests/hyperfine/normalize.json.png)

![Searching for a sequence in a fastq file \label{fig:regex}](../tests/hyperfine/regex.json.png)

![downsampling reads \label{fig:sample}](../tests/hyperfine/sample.json.png)

![sorting fastq entries by either sequence or ID \label{fig:sort}](../tests/hyperfine/sort.json.png)

![converting nonstandard fastq files to a format whose entries are four lines each, and selecting the first 100 \label{fig:straighten}](../tests/hyperfine/straighten.json.png)

## Conclusions

Fasten is a powerful manipulation suite for interleaved fastq files, written in Rust.
We benchmarked Fasten on several categories.
It has strengths as shown in Figure 1 but it does not occupy the fastest position in all cases.
Its major strengths include its competetive speeds,
Unix-style pipes,
paired-end handling,
and the advantages afforded by the Rust language including documentation and stability.

Fasten touts a comprehensive manual, continuous integration, and integration into the command line with unix pipes.
It is well poised to be a crucial module for daily work on the command line.

## Acknowledgements

Thank you, John Phan, for creating the Docker container.

## References
