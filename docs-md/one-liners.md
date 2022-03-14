# One-liners
<!-- vim-markdown-toc GFM -->

* [Read metrics for a set of files](#read-metrics-for-a-set-of-files)
* [Generate interleaved reads](#generate-interleaved-reads)
  * [Interleave split reads and feed directly to the cleaning script](#interleave-split-reads-and-feed-directly-to-the-cleaning-script)
  * [Interleave split reads, feed directly to the cleaning script, and then see how well it cleaned the reads](#interleave-split-reads-feed-directly-to-the-cleaning-script-and-then-see-how-well-it-cleaned-the-reads)
  * [Interleave split reads, randomly print 40% of the reads](#interleave-split-reads-randomly-print-40-of-the-reads)
* [read cleaning](#read-cleaning)
* [In-place fastq compression](#in-place-fastq-compression)
* [Adapter discovery](#adapter-discovery)
* [Translation to RNA](#translation-to-rna)
* [Extract a certain motif, keeping paired end reads intact](#extract-a-certain-motif-keeping-paired-end-reads-intact)
* [Mutate a genome](#mutate-a-genome)

<!-- vim-markdown-toc -->

These are random one-liners or similar that make use of Fasten.
Some or many of them are large and so they will be displayed on multiple lines for readability.

First, it might make sense to change your path, for readability.  Choose either 'debug' or 'release' at the end of the path, depending on how you compiled Fasten.rs.

    export PATH=$PATH:~/src/Fasten.rs/target/debug

## Read metrics for a set of files

    for i in *.fastq.gz; do
      echo -ne "$i\t";
      zcat $i | fasten_metrics
    done | sort | uniq | column -t

## Generate interleaved reads 

Most scripts in Fasten require interleaved reads, and so you should use `fasten_shuffle` to shuffle them.  The following example goes a step further with `grep` to help show you that the reads are interleaved.

    cat testdata/R1.fastq testdata/R2.fastq | \
      fasten_shuffle | \
      grep '^@read'

### Interleave split reads and feed directly to the cleaning script 

    cat testdata/R1.fastq testdata/R2.fastq | \
      fasten_shuffle | \
      fasten_clean --min-trim-quality 30

### Interleave split reads, feed directly to the cleaning script, and then see how well it cleaned the reads

    cat testdata/R1.fastq testdata/R2.fastq | \
      fasten_shuffle | \
      fasten_clean --min-trim-quality 30 | \
      fasten_metrics --each-read

### Interleave split reads, randomly print 40% of the reads

    cat testdata/R1.fastq testdata/R2.fastq | \
      fasten_shuffle | \
      fasten_sample --paired-end --frequency 0.4

## read cleaning

Fasten cleans reads by trimming and filtering.  View those options by running `fasten_clean --help`

    zcat some_file.fastq.gz | \
      fasten_clean --min-trim-quality 30 --min-avg-quality 20 --min-length 50 | \
      gzip -c > cleaned.fastq.gz

## In-place fastq compression

This works by sorting by GC content, which plays into how the gzip algorithm works.
`fasten_shuffle.pl` is used to shuffle the forward and reverse reads and then used to
deshuffle them.  The actual reads remain the same but are sorted differently, yielding
an estimated 10-30% compression gain.  Please see this blog for more details.  http://thegenomefactory.blogspot.com.au/2012/11/sorting-fastq-files-by-sequence-length.html?m=1

    cat testdata/R1.fastq testdata/R2.fastq | fasten_shuffle | \
      paste - - - - - - - - | \
      perl -F'\t' -lane '@GC=("$F[1]$F[5]"=~/[GCgc]/g); print join("\t",scalar(@GC)/length("$F[1]$F[5]"), @F);' | \
      sort -k1,1n | \
      cut -f2- | \
      tr '\t' '\n' | \
      fasten_shuffle --deshuffle -1 1.fastq -2 2.fastq
    gzip 1.fastq 2.fastq

Next, run `ls -lhS` on the original and sorted reads to check their size.

## Adapter discovery

What adapters are being used for your reads?  Maybe you should trim them!  Many adapters are 19mers but they can be different lengths.

    zcat file.fastq.gz | \
      fasten_trimmer --last-base 19 | \
      fasten_kmer --kmer-length 19 | \
      sort -k2,2nr | head -n 1

    => ATCGGAAGAGCACACGTCT	146

Or why not just try many kmer lengths?

    for length in $(seq 6 65); do 
      zcat file.fastq.gz | \
        fasten_trimmer --last-base $length | \
        fasten_kmer --kmer-length $length | \
        sort -k2,2nr | \
        head -n 2; 
      done;

    => CCGGCG  1623
    ...
       ATCGGAAGAGCACACGTCTGAACTCCAGTCACGTGGCCTTATCTCGTATGCCGTCTTCTGCTTGA       24

Want it to go faster by subsampling?  Use `fasten_sample --frequency 0.1` to go 10x faster.

    for length in $(seq 6 65); do 
      zcat file.fastq.gz | \
        fasten_sample --frequency 0.1 |\
        fasten_trimmer --last-base $length | \
        fasten_kmer --kmer-length $length | \
        sort -k2,2nr | \
        head -n 2; 
      done;
    
## Translation to RNA

    zcat dna.fastq.gz | fasten_replace --find 'T' --replace 'U' | \
      gzip -c > rna.fastq.gz

## Extract a certain motif, keeping paired end reads intact

    zcat file.fastq.gz | fasten_regex --regex 'ATG' --paired-end | \
      gzip -c > start-sites.fastq.gz

Choose a few different motifs with regex magic

    zcat file.fastq.gz | fasten_regex --regex 'ATG|GTG|TTG' --paired-end | \
      gzip -c > start-sites.fastq.gz
    
    zcat file.fastq.gz | fasten_regex --regex '[AGT]TG' --paired-end | \
      gzip -c > start-sites.fastq.gz

## Mutate a genome

Although you could mutate a fastq file randomly with `fasten_mutate --snps`,
it would be too random and would only cause a messy pileup or assembly
downstream.

Also, you might want to mutate a genome assembly. However, that is not why
you are browsing the fasten package. This package is for raw reads
and not assemblies.

For raw reads, it might be smarter to mutate certain loci, represented by kmers in the genome.
In this case, we use `fasten_replace` in a loop.
This example is mostly untested and so please test it before using it in production.

    k=15
    read_len=250-1;
    NTs="ATCG"
    for kmer in $kmer_array; do 
      echo -ne . >&2 # progress bar
      pos=$(shuf -i 1-$read_len -n 1)
      nt=${NT:$(shuf -i 0-3 -n 1):1} 
      replace_str=${kmer:0:$pos}$nt${kmer:$(($pos+1))}
      zcat $interleaved | \
        fasten_replace --paired-end --find $kmer --replace $replace_str --which SEQ |\
        gzip -c > $interleaved.tmp && mv $interleaved.tmp $interleaved;
    done  
    echo >&2 # finish the progress bar

