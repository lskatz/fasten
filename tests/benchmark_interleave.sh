#!/bin/bash

set -e
set -u

thisDir=$(dirname $0);
source "$thisDir/lib/benchmark.sh";
export PATH=$thisDir/../target/release:$PATH

hyperfine --export-json=$reportsDir/interleave.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "fasten shuffle" "cat $large_R1 $large_R2 | fasten_shuffle" \
  -n "seqfu interleave" "seqfu ilv -1 $large_R1 -2 $large_R2" \
  -n "fasten deshuffle" "zcat $large_interleaved | fasten_shuffle -d" \
  -n "seqfu deinterleave" "zcat $large_interleaved | seqfu dei - de-interleaved"

plot_whisker.py --title "interleave (reps=$num_runs)" --output $reportsDir/interleave.json.png $reportsDir/interleave.json

