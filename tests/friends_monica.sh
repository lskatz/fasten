#!/bin/bash

set -e

INPUT=testdata/four_reads.pe.fastq;
reads_not_cleaned=$(./target/debug/friends_monica --paired-end < $INPUT)
original_reads=$(cat $INPUT)
if [ "$reads_not_cleaned" != "$original_reads" ]; then
  echo "Test failed for not cleaning reads"
  exit 1;
fi

one_cleaned_read=$(./target/debug/friends_monica --paired-end --min-trim-quality 30 --min-length 94 < testdata/four_reads.pe.fastq | friends_rachel --each-read | tail -n 1)
if [ "$one_cleaned_read" != $'read3/2\t94\t18.446808' ]; then
  echo "Test failed for cleaning reads"
  exit 1
fi

echo "friends_monica passed"

