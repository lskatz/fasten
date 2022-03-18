#!/bin/bash

set -e

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

# Fasten_progress time trial
hyperfine --warmup 2 --shell /bin/bash --runs 10 \
  -n "Basic shuffle" "cat $large_R1 $large_R2 | fasten_shuffle > /dev/null" \
  -n "Shuffle with progress bar" "cat $large_R1 $large_R2 | fasten_progress --print 2>/dev/null | fasten_shuffle > /dev/null" \
  -n "Shuffle with wc -l" "cat $large_R1 $large_R2 | tee >(wc -l >&2) | fasten_shuffle > /dev/null"

# Fasten sort file size
zcat $large_interleaved | fasten_sort --sort-by GC --paired-end | gzip -c > $large_sorted
ls -lh $large_interleaved $large_sorted

# Convert fastq=>fasta.
# Set the width to really long on fq2fa to match the output exactly
hyperfine --warmup 2 --shell /bin/bash --runs 10 \
  -n "Fasten convert" "zcat $large_sorted | fasten_convert -i fastq -o fasta > /dev/null" \
  -n "Seqkit fq2fa -w 1000" "zcat $large_sorted | seqkit fq2fa > /dev/null"

# sorting
hyperfine --warmup 2 --shell /bin/bash --runs 10 \
  -n "Fasten sort id" "zcat $large_R1 | fasten_sort --sort-by id > /dev/null" \
  -n "Seqkit sort id" "zcat $large_R1 | seqkit sort > /dev/null"
hyperfine --warmup 2 --shell /bin/bash --runs 10 \
  -n "Fasten sort seq" "zcat $large_R1 | fasten_sort --sort-by seq > /dev/null" \
  -n "Seqkit sort seq" "zcat $large_R1 | seqkit sort --by-seq > /dev/null"

