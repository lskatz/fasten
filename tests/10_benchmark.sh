#!/bin/bash

set -e

thisDir=$(dirname $0);
export PATH=$thisDir/../target/release:$PATH

# Hyperfine parameters
# Locally, just run one time per test but in the cloud, boost it to ten
num_runs=10
if [[ ! -z "$CI" ]]; then
  num_runs=10
fi
# output directory for markdown files
reportsDir="$thisDir/hyperfine"

thisDir=$(dirname $(realpath $0))

R1="$thisDir/../testdata/R1.fastq"
R2="$thisDir/../testdata/R2.fastq"

large_R1="$thisDir/../testdata/R1.large.fastq"
large_R2="$thisDir/../testdata/R2.large.fastq"

large_interleaved="$thisDir/../testdata/shuffled.large.fastq.gz"
large_sorted="$thisDir/../testdata/shuffled.sorted.fastq.gz"

multiplier=100000;
R1_content=`cat $R1`;
R2_content=`cat $R2`;
for i in `seq 1 $multiplier`; do 
  echo "$R1_content"
done > $large_R1
for i in `seq 1 $multiplier`; do 
  echo "$R2_content"
done > $large_R2

# Version information
seqtk 2>&1 | grep -i version | sed 's/^/seqtk /'
seqkit version | grep -m 1 v
fasten_clean --version

# Save this file even though it's not used in the first benchmark
cat $R1 $R2 | fasten_shuffle | gzip -c9 > $large_interleaved
ls -lhS $large_interleaved $R1 $R2

# Fasten_progress time trial
hyperfine --export-json=$reportsDir/progress.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Basic fasten_shuffle" "cat $large_R1 $large_R2 | fasten_shuffle > /dev/null" \
  -n "Shuffle with progress bar before" "cat $large_R1 $large_R2 | fasten_progress --print 2>/dev/null | fasten_shuffle > /dev/null" \
  -n "Shuffle with progress bar after" "cat $large_R1 $large_R2 | fasten_shuffle | fasten_progress 2>/dev/null > /dev/null" 

hyperfine --export-json=$reportsDir/sample.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Fasten sample" "cat $large_R1 | fasten_sample --frequency 0.1" \
  -n "Seqtk sample" "seqtk seq -f 0.1 $large_R1"; 

# Convert fastq=>fasta.
# Set the width to really long on fq2fa to match the output exactly
hyperfine --export-json=$reportsDir/convertToFasta.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Fasten convert" "zcat $large_sorted | fasten_convert -i fastq -o fasta " \
  -n "Seqkit fq2fa -w 1000" "zcat $large_sorted | seqkit fq2fa "
  #-n "Seqtk seq -A" "seqtk seq -A $large_sorted"

# get first 100 reads for any fastq file
hyperfine --export-json=$reportsDir/straighten.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "seqkit head" "cat $large_R1 | seqkit head --number 100" \
  -n "fasten_straighten" "cat $large_R1 | fasten_straighten | head -n 400" \
  -n "seqtk seq straighten" "seqtk seq -l0 $large_R1 | head -n 400"

# Grep for CCCC
pattern="CCCC"
hyperfine --export-json=$reportsDir/regex.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "seqkit grep" "cat $large_R1 | seqkit grep --by-seq --pattern 'CCCC' > /dev/null" \
  -n "fasten_regex" "cat $large_R1 | fasten_regex --regex 'CCCC' --which SEQ --numcpus 12 > /dev/null"

# Fasten sort file size
zcat $large_interleaved | fasten_sort --sort-by GC --paired-end | gzip -c > $large_sorted
ls -lh $large_interleaved $large_sorted

# sorting
hyperfine --export-json=$reportsDir/sortById.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Fasten sort id" "zcat $large_R1 | fasten_sort --sort-by id > /dev/null" \
  -n "Seqkit sort id" "zcat $large_R1 | seqkit sort > /dev/null"
hyperfine --export-json=$reportsDir/SortBySeq.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Fasten sort seq" "zcat $large_R1 | fasten_sort --sort-by seq > /dev/null" \
  -n "Seqkit sort seq" "zcat $large_R1 | seqkit sort --by-seq > /dev/null"

