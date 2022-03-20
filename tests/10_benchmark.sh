#!/bin/bash

set -e

# Hyperfine parameters
# Locally, just run one time per test but in the cloud, boost it to ten
num_runs=1
if [[ ! -z "$CI" ]]; then
  num_runs=10
fi
# output directory for markdown files
reportsDir="hyperfine"

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

# Save this file even though it's not used in the first benchmark
cat $R1 $R2 | fasten_shuffle | gzip -c9 > $large_interleaved
ls -lhS $large_interleaved $R1 $R2

hyperfine --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Fasten sample" "cat $large_R1 | fasten_sample --frequency 0.1" \
  -n "Seqtk sample" "seqtk seq -f 0.1 $large_R1"; 

# Convert fastq=>fasta.
# Set the width to really long on fq2fa to match the output exactly
hyperfine --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Fasten convert" "zcat $large_sorted | fasten_convert -i fastq -o fasta " \
  -n "Seqkit fq2fa -w 1000" "zcat $large_sorted | seqkit fq2fa " \
  -n "Seqtk seq -A" "seqtk seq -A $large_sorted"

# get first 100 reads for any fastq file
hyperfine --warmup 2 --shell $SHELL --runs $num_runs --export-markdown $(mktemp --tmpdir=$reportsDir hyperfine.XXXXXX.md) \
  -n "seqkit head" "cat $large_R1 | seqkit head --number 100" \
  -n "fasten_straighten" "cat $large_R1 | fasten_straighten | head -n 400"

# Grep for CCCC
pattern="CCCC"
hyperfine --warmup 2 --shell $SHELL --runs $num_runs \
  -n "seqkit grep" "cat $large_R1 | seqkit grep --by-seq --pattern 'CCCC' > /dev/null" \
  -n "fasten_regex" "cat $large_R1 | fasten_regex --regex 'CCCC' --which SEQ --numcpus 12 > /dev/null"

# Fasten_progress time trial
hyperfine --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Basic fasten_shuffle" "cat $large_R1 $large_R2 | fasten_shuffle > /dev/null" \
  -n "Shuffle with progress bar" "cat $large_R1 $large_R2 | fasten_progress --print 2>/dev/null | fasten_shuffle > /dev/null" \
  -n "Shuffle with wc -l" "cat $large_R1 $large_R2 | tee >(wc -l >&2) | fasten_shuffle > /dev/null"

# Fasten sort file size
zcat $large_interleaved | fasten_sort --sort-by GC --paired-end | gzip -c > $large_sorted
ls -lh $large_interleaved $large_sorted

# sorting
hyperfine --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Fasten sort id" "zcat $large_R1 | fasten_sort --sort-by id > /dev/null" \
  -n "Seqkit sort id" "zcat $large_R1 | seqkit sort > /dev/null"
hyperfine --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Fasten sort seq" "zcat $large_R1 | fasten_sort --sort-by seq > /dev/null" \
  -n "Seqkit sort seq" "zcat $large_R1 | seqkit sort --by-seq > /dev/null"

