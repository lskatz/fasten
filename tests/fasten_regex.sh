#!/bin/bash

set -e

INPUT=testdata/four_reads.pe.fastq
REGEX="TTTT"

all_reads=$(cat $INPUT)
no_filtered=$(echo "$all_reads" | ./target/debug/fasten_regex --regex .)
se_filtered=$(echo "$all_reads" | ./target/debug/fasten_regex --regex $REGEX)
pe_filtered=$(echo "$all_reads" | ./target/debug/fasten_regex --regex $REGEX --paired-end)

r1_filtered=$(echo "$all_reads" | ./target/debug/fasten_regex --regex read1 --which ID)

if [ "$all_reads" != "$no_filtered" ]; then
  echo "ERROR using regex to filter for all reads"
  exit 1
fi

if [ "$(wc -l <<< "$se_filtered")" -ne 12 ]; then
  echo "ERROR single-end filtering reads for only those with $REGEX"
  exit 1
fi
if [ "$(wc -l <<< "$pe_filtered")" -ne 24 ]; then
  echo "ERROR paired-end filtering reads for only those with $REGEX"
  exit 1
fi

if [ "$(echo "$pe_filtered" | ./target/debug/fasten_metrics --each-read | tail -n +2 | cut -f 1 |  awk '{if(NR>1) printf "+"; printf $1}' )" != "read0/1+read0/2+read1/1+read1/2+read2/1+read2/2" ]; then
  echo "ERROR filtering for the right read names"
  exit 1
fi

if [ "$(echo "$r1_filtered" | ./target/debug/fasten_regex --regex read1 --which ID | ./target/debug/fasten_metrics --each-read | tail -n +2 | cut -f 1 |  awk '{if(NR>1) printf "+"; printf $1}' )" != "read1/1+read1/2" ]; then
  echo "ERROR running regex on IDs for read1"
  exit 1
fi

echo "$0 passed!"
