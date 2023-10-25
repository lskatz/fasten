#!/bin/bash

set -e

thisDir=$(dirname $0);
export PATH=$thisDir/../target/release:$PATH

# Hyperfine parameters
# Locally, just run a handful of times per test
# but in the cloud, boost it to ten
num_runs=20
if [[ ! -z "$CI" ]]; then
  num_runs=100
fi
# output directory for markdown files
reportsDir="$thisDir/hyperfine"
mkdir -pv $reportsDir

thisDir=$(dirname $(realpath $0))

R1="$thisDir/../testdata/R1.fastq"
R2="$thisDir/../testdata/R2.fastq"

large_R1="$thisDir/../testdata/R1.large.fastq"
large_R2="$thisDir/../testdata/R2.large.fastq"

large_interleaved="$thisDir/../testdata/shuffled.large.fastq.gz"
large_sorted="$thisDir/../testdata/shuffled.sorted.fastq.gz"

# Just make it simple locally but a large number online
multiplier=1000
if [[ ! -z "$CI" ]]; then
  multiplier=100000;
fi
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

plot_whisker.py --title "Interleave reads with or without a progress meter" --labels without,before,after --output $reportsDir/progress.json.png $reportsDir/progress.json

hyperfine --export-json=$reportsDir/sample.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Fasten sample" "cat $large_R1 | fasten_sample --frequency 0.1" \
  -n "Seqtk sample" "seqtk seq -f 0.1 $large_R1"; 

plot_whisker.py --title "subsample reads" --labels "fasten sample, seqtk sample" --output $reportsDir/sample.json.png $reportsDir/sample.json

# Convert fastq=>fasta.
# Set the width to really long on fq2fa to match the output exactly
hyperfine --export-json=$reportsDir/convertToFasta.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Fasten convert" "zcat $large_sorted | fasten_convert -i fastq -o fasta " \
  -n "Seqkit fq2fa -w 1000" "zcat $large_sorted | seqkit fq2fa "
  #-n "Seqtk seq -A" "seqtk seq -A $large_sorted"

plot_whisker.py --title "Converting fastq to fasta" --output $reportsDir/convertToFasta.json.png $reportsDir/convertToFasta.json

# get first 100 reads for any fastq file
hyperfine --export-json=$reportsDir/straighten.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "seqkit head" "cat $large_R1 | seqkit head --number 100" \
  -n "fasten_straighten" "cat $large_R1 | fasten_straighten | head -n 400" \
  -n "seqtk seq straighten" "seqtk seq -l0 $large_R1 | head -n 400"

plot_whisker.py --title "Get first 100 reads from fastq" --output $reportsDir/straighten.json.png $reportsDir/straighten.json

# Grep for seq 
pattern="CCCC"
pattern2="ATG"
hyperfine --export-json=$reportsDir/regex.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "seqkit grep $pattern" "cat $large_R1 | seqkit grep --by-seq --pattern '$pattern' > /dev/null" \
  -n "seqkit grep $pattern2" "cat $large_R1 | seqkit grep --by-seq --pattern '$pattern2' > /dev/null" \
  -n "fasten_regex $pattern" "cat $large_R1 | fasten_regex --regex '$pattern' --which SEQ > /dev/null" \
  -n "fasten_regex $pattern2" "cat $large_R1 | fasten_regex --regex '$pattern2' --which SEQ > /dev/null"

plot_whisker.py --title "Regex on a sequence" --output $reportsDir/regex.json.png $reportsDir/regex.json

# Fasten sort file size
zcat $large_interleaved | fasten_sort --sort-by GC --paired-end | gzip -c > $large_sorted
ls -lh $large_interleaved $large_sorted

# sorting
hyperfine --export-json=$reportsDir/sort.json --warmup 2 --shell $SHELL --runs $num_runs \
  -n "Fasten sort id" "zcat $large_R1 | fasten_sort --sort-by id > /dev/null" \
  -n "Fasten sort seq" "zcat $large_R1 | fasten_sort --sort-by seq > /dev/null" \
  -n "Seqkit sort id" "zcat $large_R1 | seqkit sort > /dev/null" \
  -n "Seqkit sort seq" "zcat $large_R1 | seqkit sort --by-seq > /dev/null"

plot_whisker.py --title "Sort sequences" --output $reportsDir/sort.json.png $reportsDir/sort.json

