#!/bin/bash

set -e
set -u

thisDir=$(dirname $0);
source "$thisDir/lib/benchmark.sh";
export PATH=$thisDir/../target/release:$PATH

metrics_small=$(cat $R1 | fasten_metrics | tail -n 1 | cut -f 2)
metrics_large=$(cat $large_R1 | fasten_metrics | tail -n 1 | cut -f 2)
metrics_largest=$(zcat $largest | fasten_metrics | tail -n 1 | cut -f 2)

hyperfine --export-json=$reportsDir/metrics.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "(nreads=$metrics_small) fasten_metrics" "cat R1 | fasten_metrics" \
  -n "(nreads=$metrics_small) seqkit stats -a" "cat R1 | seqkit stats -a" \
  -n "(nreads=$metrics_small) seqtk fqchk" "cat R1 | seqtk fqchk -q0 -" \
  -n "(nreads=$metrics_large) fasten_metrics" "cat $large_R1 | fasten_metrics" \
  -n "(nreads=$metrics_large) seqkit stats -a" "cat $large_R1 | seqkit stats -a" \
  -n "(nreads=$metrics_large) seqtk fqchk" "cat $large_R1 | seqtk fqchk -q0 -" \
  -n "(nreads=$metrics_largest) fasten_metrics" "zcat $largest | fasten_metrics" \
  -n "(nreads=$metrics_largest) seqkit stats -a" "zcat $largest | seqkit stats -a" \
  -n "(nreads=$metrics_largest) seqtk fqchk" "zcat $largest | seqtk fqchk -q0 -"

title="Metrics (reps=$num_runs) (note: softwares are not an exact 1:1 equivalent)"

plot_whisker.py --title "$title" \
  --output $reportsDir/metrics.json.png $reportsDir/metrics.json
