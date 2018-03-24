#!/bin/bash

set -e

INPUT=testdata/four_reads.pe.fastq

top_kmer=$(./target/debug/friends_chandler --kmer-length 3 < $INPUT | sort -k1,1 | head -n 1)

if [ "$top_kmer" != $'AAA\t19' ]; then
  echo "Chandler did not count kmers correctly"
  exit 1
fi

echo "Chandler passed!"
