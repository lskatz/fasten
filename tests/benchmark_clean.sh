#!/bin/bash

set -e
set -u

thisDir=$(dirname $0);
source "$thisDir/lib/benchmark.sh";
export PATH=$thisDir/../target/release:$PATH

# get first 100 reads for any fastq file
hyperfine --export-json=$reportsDir/clean.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "fasten_clean" "cat $large_R1 | fasten_clean --min-length 0 --min-trim-quality 5" \
  -n "seqtk trimfq" "cat $large_R1 | seqtk trimfq -q 0.1 -"

plot_whisker.py --title "Clean (reps=$num_runs)" --output $reportsDir/clean.json.png $reportsDir/clean.json

#  -n "friends_monica.pl" "friends_monica.pl  -i $large_R1 -o /dev/stdout" \
