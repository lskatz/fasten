#!/bin/bash

set -e

INPUT=testdata/four_reads.pe.fastq;
reads_not_cleaned=$(./target/debug/friends_monica --paired-end < $INPUT)
original_reads=$(cat $INPUT)
if [ "$reads_not_cleaned" != "$original_reads" ]; then
  echo "Test failed for not cleaning reads"
  exit 1;
fi


echo "friends_monica passed"

