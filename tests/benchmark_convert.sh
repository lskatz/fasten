#!/bin/bash

set -e
set -u

thisDir=$(dirname $0);
source "$thisDir/lib/benchmark.sh";
export PATH=$thisDir/../target/release:$PATH

ls -lh $large_sorted

# Convert fastq=>fasta.
# Set the width to really long on fq2fa to match the output exactly
hyperfine --export-json=$reportsDir/convertToFasta.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Fasten convert" "zcat $large_sorted | fasten_convert -i fastq -o fasta " \
  -n "Seqkit fq2fa -w 0" "zcat $large_sorted | seqkit fq2fa " \
  -n "Seqtk seq -A" "zcat $large_sorted | seqtk seq -A - " \
  -n "fastx convert" "zcat $large_sorted | fastq_to_fasta -i - -o - -Q33" 

plot_whisker.py --title "Converting fastq to fasta (reps=$num_runs)" --output $reportsDir/convertToFasta.json.png $reportsDir/convertToFasta.json

