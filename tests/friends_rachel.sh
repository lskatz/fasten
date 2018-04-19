#!/bin/bash

set -e

avg_qual=$(./target/debug/friends_rachel < testdata/four_reads.pe.fastq | cut -f 4 |tail -n 1)
if [ "$avg_qual" != "19.53875" ]; then
  echo "Did not receive the correct average quality score"
  exit 1;
fi

first_read_metrics=$(./target/debug/friends_rachel --each-read < testdata/four_reads.pe.fastq | sed -n 2p)
if [ "$first_read_metrics" != $'read0/1\t100\t20.93' ]; then
  echo "Did not receive the correct metrics for R1 of the first read"
  exit 1;
fi

echo "friends_rachel passed"

