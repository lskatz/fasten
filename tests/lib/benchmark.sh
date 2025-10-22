#!/bin/bash

set -e
set -u

# Hyperfine parameters
# Locally, just run a handful of times per test
# but in the cloud, boost it to ten
num_runs=1000 
# How many times to multiply the four reads file to make a large one
multiplier=10

if [[ -z ${thisDir+x} ]]; then
  echo "ERROR: \$thisDir needs to be set up in a shell script, and then this library should be sourced."
  exit 1
fi

# output directory for markdown files
reportsDir="$thisDir/hyperfine"
mkdir -pv $reportsDir

R1="$thisDir/../testdata/R1.fastq"
R2="$thisDir/../testdata/R2.fastq"

large_R1="$thisDir/../testdata/R1.large.fastq"
large_R2="$thisDir/../testdata/R2.large.fastq"

large_interleaved="$thisDir/../testdata/shuffled.large.fastq.gz"
large_sorted="$thisDir/../testdata/shuffled.sorted.fastq.gz"

largest="$thisDir/../testdata/shuffled.sorted.largest.fastq.gz"

if [[ ! -s $large_R1 || ! -s $large_R2 ]]; then
  R1_content=`cat $R1`;
  R2_content=`cat $R2`;
  for i in `seq 1 $multiplier`; do 
    echo "$R1_content"
  done | seqkit rename > $large_R1
  for i in `seq 1 $multiplier`; do 
    echo "$R2_content"
  done | seqkit rename > $large_R2
fi

if [[ ! -s $large_interleaved ]]; then
  # Create large interleaved file if not present
  cat $large_R1 $large_R2 | fasten_shuffle | gzip -c9 > $large_interleaved
fi

if [[ ! -s $large_sorted ]]; then
  # Create large sorted file if not present
  zcat $large_interleaved | fasten_sort --sort-by GC --paired-end | gzip -c > $large_sorted
fi

if [[ ! -s $largest ]]; then
  # Create largest sorted file if not present
  for i in `seq 1 $multiplier`; do
    for j in `seq 1 $multiplier`; do
      cat $large_sorted
    done
  done > $largest
fi

which bbnorm.sh
which fasten_clean
which seqfu

# Version information
seqtk 2>&1 | grep -i version | sed 's/^/seqtk /'
seqkit version | grep -m 1 v
fasten_clean --version
fastq_to_fasta -h | grep "Part of FASTX"
bbnorm.sh version 2>&1 | grep -E '(BBMap|BBTools) version'
seqfu --version

# hyperfine
which hyperfine
which plot_whisker.py || echo "WARNING: plot_whisker.py from hyperfine not found in path: will not be able to plot graphs."

