#!/bin/bash

set -e

avg_qual=$(./target/debug/fasten_metrics < testdata/four_reads.pe.fastq | cut -f 4 |tail -n 1)
if [ "$avg_qual" != "8.88" ]; then
  echo "Did not receive the correct average quality score. Got $avg_qual"
  exit 1;
fi

first_read_metrics=$(./target/debug/fasten_metrics --each-read < testdata/four_reads.pe.fastq | sed -n 2p)
if [ "$first_read_metrics" != $'read0/1\t100\t10.00' ]; then
  echo "Did not receive the correct metrics for R1 of the first read. Got $first_read_metrics"
  exit 1;
fi

echo "fasten_metrics passed"

