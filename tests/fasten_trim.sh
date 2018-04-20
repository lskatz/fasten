#!/bin/bash

set -e

INPUT=testdata/four_reads.pe.fastq;
reads_not_trimmed=$(./target/debug/fasten_trimmer < $INPUT )
original_reads=$(cat $INPUT)
if [ "$reads_not_trimmed" != "$original_reads" ]; then
  echo "ERROR while not trimming reads"
  exit 1
fi

onebase=$(./target/debug/fasten_trimmer --first-base 3 --last-base 4 < testdata/four_reads.pe.fastq | perl -lane 'print if($i++ % 4 == 1);' | paste -sd'_')
shouldbe="T_T_G_A_C_A_C_A"
if [ "$onebase" != "$shouldbe" ]; then
  echo "ERROR trimming to the third base"
  exit 1
fi

last_read_length=$(./target/debug/fasten_trimmer --first-base 53 < testdata/four_reads.pe.fastq | ./target/debug/fasten_metrics --each-read | tail -n 1 | cut -f 2)
if [ "$last_read_length" -ne 47 ]; then
  echo "ERROR trimming to the last 47 bp of the reads"
  exit 1
fi

echo "$0 passed";
