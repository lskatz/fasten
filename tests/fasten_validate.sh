#!/bin/bash

set -e

INPUT=testdata/four_reads.pe.fastq;
./target/debug/fasten_validate --paired-end < $INPUT
if [ $? -gt 0 ]; then
  echo "Test failed for validating a good set of reads"
  exit 1
fi

set +e
head -n 15 $INPUT | ./target/debug/fasten_validate --paired-end --verbose 2>/dev/null
if [ $? -eq 0 ]; then
  echo "Test failed for marking a bad set of reads"
  exit 1
fi
set -e

echo "$0 passed!"
