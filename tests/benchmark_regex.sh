#!/bin/bash

set -e
set -u

thisDir=$(dirname $0);
source "$thisDir/lib/benchmark.sh";
export PATH=$thisDir/../target/release:$PATH

# Grep for seq 
pattern="CCCC"
pattern2="ATG"
hyperfine --export-json=$reportsDir/regex.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "fasten_regex $pattern" "cat $large_R1 | fasten_regex --regex '$pattern' --which SEQ" \
  -n "fasten_regex $pattern2" "cat $large_R1 | fasten_regex --regex '$pattern2' --which SEQ" \
  -n "seqkit grep $pattern" "cat $large_R1 | seqkit grep --by-seq --pattern '$pattern'" \
  -n "seqkit grep $pattern2" "cat $large_R1 | seqkit grep --by-seq --pattern '$pattern2'" \
  -n "seqfu grep $pattern" "seqfu grep -r '$pattern' $large_R1" \
  -n "seqfu grep $pattern2" "seqfu grep -r '$pattern2' $large_R1" \

plot_whisker.py --title "Regex on a sequence (reps=$num_runs)" --output $reportsDir/regex.json.png $reportsDir/regex.json

