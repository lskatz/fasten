#!/bin/bash

set -e
set -u

thisDir=$(dirname $0);
source "$thisDir/lib/benchmark.sh";
export PATH=$thisDir/../target/release:$PATH

which jellyfish

trap ' { rm jf.jf; } ' EXIT

# get first 100 reads for any fastq file
hyperfine --export-json=$reportsDir/kmer.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "fasten_kmer k=11" "cat $large_R1 | fasten_kmer --kmer-length 11" \
  -n "jellyfish k=11" "jellyfish count -m 11 -s 1000000 -o jf.jf $large_R1 && jellyfish dump --column --tab jf.jf" \
  -n "fasten_kmer k=25" "cat $large_R1 | fasten_kmer --kmer-length 25" \
  -n "jellyfish k=25" "jellyfish count -m 25 -s 1000000 -o jf.jf $large_R1 && jellyfish dump --column --tab jf.jf" \
  -n "fasten_kmer k=31" "cat $large_R1 | fasten_kmer --kmer-length 31" \
  -n "jellyfish k=31" "jellyfish count -m 31 -s 1000000 -o jf.jf $large_R1 && jellyfish dump --column --tab jf.jf" \
  -n "fasten_kmer k=51" "cat $large_R1 | fasten_kmer --kmer-length 51" \
  -n "jellyfish k=51" "jellyfish count -m 51 -s 1000000 -o jf.jf $large_R1 && jellyfish dump --column --tab jf.jf" 


plot_whisker.py --title "Kmer counting (reps=$num_runs)" --output $reportsDir/kmer.json.png $reportsDir/kmer.json

