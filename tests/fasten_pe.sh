#!/bin/bash

set -e

INPUT_SRA=testdata/four_reads.pe.fastq
INPUT_MISEQ=testdata/miseq.pe.fastq
INPUT_SE=testdata/four_reads.fastq

SRA_READS=$(cat $INPUT_SRA)
MISEQ_READS=$(cat $INPUT_MISEQ)

for i in $(seq 1 100); do
  echo "$SRA_READS"
done | ./target/debug/fasten_pe

for i in $(seq 1 100); do
  echo "$MISEQ_READS"
done | ./target/debug/fasten_pe

BAD_RESPONSE=$(cat $INPUT_SE | ./target/debug/fasten_pe || echo "FAILED READS")
if [ "$BAD_RESPONSE" != "FAILED READS" ]; then
  echo "ERROR with $0: did not detect single end reads"
  exit 1
fi

echo "$0 passed!"
