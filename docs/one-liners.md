# One-liners

These are random one-liners or similar that make use of ROSS.
Some or many of them are large and so they will be displayed on multiple lines for readability.

First, it might make sense to change your path, for readability.  Choose either 'debug' or 'release' at the end of the path, depending on how you compiled ROSS.rs.

    export PATH=$PATH:~/src/ROSS.rs/target/debug

## Generate interleaved reads 

![Joey](/images/joey.png)

Most scripts in ROSS require interleaved reads, and so you should use `friends_joey` to shuffle them.  The following example goes a step further with `grep` to help show you that the reads are interleaved.

    cat testdata/R1.fastq testdata/R2.fastq | \
      friends_joey | \
      grep '^@read'

### Interleave split reads and feed directly to the cleaning script 

![Joey](/images/joey.png) => ![Monica](/images/monica.jpg)

    cat testdata/R1.fastq testdata/R2.fastq | \
      friends_joey | \
      friends_monica --min-trim-quality 30

### Interleave split reads, feed directly to the cleaning script, and then see how well it cleaned the reads

![Joey](/images/joey.png) => ![Monica](/images/monica.jpg) => ![Rachel](/images/rachel.jpg)

    cat testdata/R1.fastq testdata/R2.fastq | \
      friends_joey | \
      friends_monica --min-trim-quality 30 | \
      friends_rachel --each-read

## read cleaning

![Monica](/images/monica.jpg)

ROSS cleans reads by trimming and filtering.  View those options by running `friends_monica --help`

    zcat some_file.fastq.gz | \
      friends_monica --min-trim-quality 30 --min-avg-quality 20 --min-length 50 | \
      gzip -c > cleaned.fastq.gz

## In-place fastq compression

This works by sorting by GC content, which plays into how the gzip algorithm works.
`friends_joey.pl` is used to shuffle the forward and reverse reads and then used to
deshuffle them.  The actual reads remain the same but are sorted differently, yielding
an estimated 10-30% compression gain.  Please see this blog for more details.  http://thegenomefactory.blogspot.com.au/2012/11/sorting-fastq-files-by-sequence-length.html?m=1

    cat testdata/R1.fastq testdata/R2.fastq | friends_joey | \
      paste - - - - - - - - | \
      perl -F'\t' -lane '@GC=("$F[1]$F[5]"=~/[GCgc]/g); print join("\t",scalar(@GC)/length("$F[1]$F[5]"), @F);' | \
      sort -k1,1n | \
      cut -f2- | \
      tr '\t' '\n' | \
      friends_joey --deshuffle -1 1.fastq -2 2.fastq
    gzip 1.fastq 2.fastq

Next, run `ls -lhS` on the original and sorted reads to check their size.

## Adapter discovery

What adapters are being used for your reads?  Maybe you should trim them!  Many adapters are 19mers but they can be different lengths.

    zcat file.fastq.gz | \
      friends_trimmer --last-base 19 | \
      friends_chandler --kmer-length 19 | \
      sort -k2,2nr | head -n 1

    => ATCGGAAGAGCACACGTCT	146

Or why not just try many kmer lengths?

    for length in $(seq 6 65); do 
      zcat file.fastq.gz | \
        friends_trimmer --last-base $length | \
        friends_chandler --kmer-length $length | \
        sort -k2,2nr | \
        head -n 2; 
      done;

    => CCGGCG  1623
    ...
       ATCGGAAGAGCACACGTCTGAACTCCAGTCACGTGGCCTTATCTCGTATGCCGTCTTCTGCTTGA       24

Want it to go faster by subsampling?  Use `friends_phoebe | head` to get 1,000 random reads.

    for length in $(seq 6 65); do 
      zcat file.fastq.gz | \
        friends_phoebe |\
        head -n 8000 \
        friends_trimmer --last-base $length | \
        friends_chandler --kmer-length $length | \
        sort -k2,2nr | \
        head -n 2; 
      done;
    


