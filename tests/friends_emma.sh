#!/bin/bash

set -e

INPUT=testdata/four_reads.pe.fastq;
reads_not_collapsed=$(./target/debug/friends_emma < $INPUT | perl -lane 'print if($i++ % 2 ==1);' | sort)
original_reads=$(cat $INPUT | perl -lane 'print if($i++ %2 == 1);' | sort)
if [ "$reads_not_collapsed" != "$original_reads" ]; then
  echo "Test failed for not collapsing reads"
  echo "$reads_not_collapsed" |  md5sum
  echo "...";
  echo
  echo "$original_reads" | head
  echo "..."
  exit 1;
fi

reads_collapsed=$(cat $INPUT $INPUT $INPUT | ./target/debug/friends_emma | ./target/debug/friends_rachel --each-read)
total_quals=$(echo "$reads_collapsed" | cut -f 3 | tail -n +2 | paste -sd+ | bc -l)
if [ "$total_quals" != "259.31" ]; then
  echo "Test failed for total expected quality when collapsing three sets of reads"
  exit 1
fi

echo "$0 passed";
