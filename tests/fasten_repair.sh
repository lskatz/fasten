#!/bin/bash

set -e

INPUT=testdata/four_reads.pe.fastq;
./target/debug/fasten_inspect < $INPUT > /dev/null
if [ $? -gt 0 ]; then
  echo "Test failed for validating a good set of reads"
  exit 1
fi

set +e
head -n 15 $INPUT | ./target/debug/fasten_inspect | \
  ./target/debug/fasten_repair > /dev/null 2>&1
if [ $? -gt 0 ]; then
  echo "Test failed for repairing a truncated set of reads"
  exit 1
fi
set -e

# Paired end
./target/debug/fasten_inspect --paired-end < $INPUT > /dev/null 
if [ $? -gt 0 ]; then
  echo "Test failed for validating a good set of paired end reads"
  exit 1
fi

set +e
head -n 15 $INPUT | ./target/debug/fasten_inspect --paired-end | \
  ./target/debug/fasten_repair --paired-end --mode panic --remove-info > /dev/null 2>/dev/null
if [ $? -eq 0 ]; then
  echo "Test failed for detecting a truncated set of reads"
  exit 1
fi
set -e

head -n 15 $INPUT | ./target/debug/fasten_inspect --paired-end | \
  ./target/debug/fasten_repair --paired-end --mode repair > /dev/null 2>&1 
if [ $? -gt 0 ]; then
  echo "Test failed for repairing a truncated set of reads"
  exit 1
fi

# Test to make sure this gives 2 reads
num_reads=$(
    head -n 15 $INPUT | ./target/debug/fasten_inspect --paired-end | \
    ./target/debug/fasten_repair --paired-end --mode repair 2>/dev/null | \
    ./target/debug/fasten_metrics | cut -f 2 | tail -n 1
    ) 
if [ "$num_reads" -ne 2 ]; then
  echo "Test failed for repairing a truncated set of reads and returning exactly 2"
  exit 1
fi

echo "$0 passed!" >&2