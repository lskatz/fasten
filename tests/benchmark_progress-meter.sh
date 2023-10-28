#!/bin/bash

set -e
set -u

thisDir=$(dirname $0);
source "$thisDir/lib/benchmark.sh";
export PATH=$thisDir/../target/release:$PATH

# Fasten_progress time trial
hyperfine --export-json=$reportsDir/progress.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Basic fasten_shuffle" "cat $large_R1 $large_R2 | fasten_shuffle > /dev/null" \
  -n "Shuffle with progress bar before" "cat $large_R1 $large_R2 | fasten_progress --print 2>/dev/null | fasten_shuffle > /dev/null" \
  -n "Shuffle with progress bar after" "cat $large_R1 $large_R2 | fasten_shuffle | fasten_progress 2>/dev/null > /dev/null" 

plot_whisker.py --title "Interleave reads with or without a progress meter (reps=$num_runs)" --labels without,before,after --output $reportsDir/progress.json.png $reportsDir/progress.json

