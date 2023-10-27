#!/bin/bash

set -e
set -u

thisDir=$(dirname $0);
source "$thisDir/lib/benchmark.sh";
export PATH=$thisDir/../target/release:$PATH

tmpfile=$(mktemp --dry-run --suffix=.fastq)
trap " { rm $tmpfile; } " EXIT

# get first 100 reads for any fastq file
hyperfine --export-json=$reportsDir/normalize.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "fasten_normalize" "cat $large_R1 | fasten_kmer -k 5 --remember-reads | fasten_normalize --target-depth 50 > $tmpfile" \
  -n "BBNorm" "bbnorm.sh in=$large_R1 out=$tmpfile target=50 min=1 threads=1 k=5"

plot_whisker.py --title "Normalize kmer depth (reps=$num_runs)" --output $reportsDir/normalize.json.png $reportsDir/normalize.json

