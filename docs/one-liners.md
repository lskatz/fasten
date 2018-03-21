# One-liners

These are random one-liners or similar that make use of ROSS.
Some or many of them are large and so they will be displayed on multiple lines for readability.

First, it might make sense to change your path, for readability.  Choose either 'debug' or 'release' at the end of the path, depending on how you compiled ROSS.rs.

    export PATH=$PATH:~/src/ROSS.rs/target/debug

## Generate interleaved reads

Most scripts in ROSS require interleaved reads, and so you should use `friends_joey` to shuffle them.  The following example goes a step further with `grep` to help show you that the reads are interleaved.

    cat testdata/R1.fastq testdata/R2.fastq | \
      friends_joey | \
      grep '^@read'

### Interleave split reads and feed directly to the cleaning script

    cat testdata/R1.fastq testdata/R2.fastq | \
      friends_joey | \
      friends_monica --min-trim-quality 30

### Interleave split reads, feed directly to the cleaning script, and then see how well it cleaned the reads

    cat testdata/R1.fastq testdata/R2.fastq | \
      friends_joey | \
      friends_monica --min-trim-quality 30 | \
      friends_rachel --each-read

## read cleaning

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
      friends_joey.pl --deshuffle -1 1.fastq -2 2.fastq
    gzip 1.fastq 2.fastq

Next, run `ls -lhS` on the original and sorted reads to check their size.

