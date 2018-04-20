#!/bin/bash

set -e

INPUT=testdata/four_reads.pe.fastq;
reads_not_collapsed=$(./target/debug/fasten_combine < $INPUT | perl -lane 'print if($i++ % 2 ==1);' | sort)
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

reads_collapsed=$(cat $INPUT $INPUT $INPUT | ./target/debug/fasten_combine | ./target/debug/fasten_metrics --each-read)
total_quals=$(echo "$reads_collapsed" | cut -f 3 | tail -n +2 | paste -sd+ | bc -l)
if [ "$total_quals" != "259.31" ]; then
  echo "Test failed for total expected quality when collapsing three sets of reads"
  exit 1
fi

pe_collapsed=$(cat $INPUT $INPUT $INPUT | ./target/debug/fasten_combine --paired-end | ./target/debug/fasten_metrics --each-read)
IDs=$(echo "$pe_collapsed" | cut -f 1 | tail -n +2 | paste -sd+)
pe_quals=$(echo "$pe_collapsed" | cut -f 3 | tail -n +2 | paste -sd+ | bc -l)
if [ "$IDs" != "1/1+1/2+2/1+2/2+3/1+3/2+4/1+4/2" ]; then
  echo "Test failed for total expected quality when collapsing three sets of reads using --paired-end"
  exit 1
fi
if [ "$total_quals" != "$pe_quals" ]; then
  echo "Test failed for having the same quality scores in SE mode as PE mode"
  exit 1
fi

echo "$0 passed";
