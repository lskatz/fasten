#!/bin/bash

set -e
set -u

thisDir=$(dirname $0);
source "$thisDir/lib/benchmark.sh";
export PATH=$thisDir/../target/release:$PATH

# sorting
hyperfine --export-json=$reportsDir/sort.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Fasten sort id" "cat $large_R1 | fasten_sort --sort-by id > /dev/null" \
  -n "Fasten sort seq" "cat $large_R1 | fasten_sort --sort-by seq > /dev/null" \
  -n "Seqkit sort id" "cat $large_R1 | seqkit sort > /dev/null" \
  -n "Seqkit sort seq" "cat $large_R1 | seqkit sort --by-seq > /dev/null" \
  -n "unix pipe sort id" "cat $large_R1 | paste - - - - | sort -k 1 | tr '\t' '\n' > /dev/null" \
  -n "unix pipe sort seq" "cat $large_R1 | paste - - - - | sort -k 2 | tr '\t' '\n' > /dev/null" \

plot_whisker.py --title "Sort sequences (reps=$num_runs)" --output $reportsDir/sort.json.png $reportsDir/sort.json

