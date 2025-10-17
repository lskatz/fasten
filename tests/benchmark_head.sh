#!/bin/bash

set -e
set -u

thisDir=$(dirname $0);
source "$thisDir/lib/benchmark.sh";
export PATH=$thisDir/../target/release:$PATH

# get first 100 reads for any fastq file
hyperfine --export-json=$reportsDir/straighten.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "fasten_straighten | head" "cat $large_R1 | fasten_straighten | head -n 400" \
  -n "fasten head" "cat $large_R1 | fasten_head --reads 100" \
  -n "seqkit head" "cat $large_R1 | seqkit head --number 100" \
  -n "seqtk seq straighten" "seqtk seq -l0 $large_R1 | head -n 400" \
  -n "seqfu head" "cat $large_R1 | seqfu head --num 100"

plot_whisker.py --title "Get first 100 reads from fastq (reps=$num_runs)" --output $reportsDir/straighten.json.png $reportsDir/straighten.json

