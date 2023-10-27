#!/bin/bash

set -e
set -u

thisDir=$(dirname $0);
source "$thisDir/lib/benchmark.sh";
export PATH=$thisDir/../target/release:$PATH

hyperfine --export-json=$reportsDir/sample.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Fasten sample" "cat $large_R1 | fasten_sample --frequency 0.1" \
  -n "seqkit sample" "cat $large_R1 | seqkit sample --proportion 0.1" \
  -n "Seqtk sample" "seqtk seq -f 0.1 $large_R1"; 

plot_whisker.py --title "subsample reads (reps=$num_runs)" --labels "fasten sample,seqkit sample,seqtk sample" --output $reportsDir/sample.json.png $reportsDir/sample.json

