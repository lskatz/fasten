#!/bin/bash

set -e
set -u

thisDir=$(dirname $0);
source "$thisDir/lib/benchmark.sh";
export PATH=$thisDir/../target/release:$PATH

# get first 100 reads for any fastq file
hyperfine --export-json=$reportsDir/trim.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "fasten_trim" "cat $large_R1 | fasten_trim -f 5 -l 10" \
  -n "seqtk trimfq" "cat $large_R1 | seqtk trimfq -b 5 -e 90 -" \
  -n "seqfu cat" "cat $large_R1 | seqfu cat --trim-front 5 --trim-tail 90"

plot_whisker.py --title "Trim (reps=$num_runs)" --output $reportsDir/trim.json.png $reportsDir/trim.json

