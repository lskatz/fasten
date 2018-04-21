#!/bin/bash

set -e

INPUT=testdata/four_reads.pe.fastq

reads=$(for i in $(seq 1 1000); do cat $INPUT; done;)
not_downsampled=$(echo "$reads" | ./target/debug/fasten_sample)
ten_percent=$(echo "$reads" | ./target/debug/fasten_sample --frequency 0.1)
fifty_percent=$(echo "$reads" | ./target/debug/fasten_sample --frequency 0.5)
ninty_percent=$(echo "$reads" | ./target/debug/fasten_sample --frequency 0.9)

if [ "$reads" != "$not_downsampled" ]; then
  echo "Test failed for not downsampling"
  exit 1
fi

if [ "$(wc -l <<< $reads)" -lt "$(wc -l <<< $ninty_percent)" ]; then
  echo "Test failed for downsampling to 90%"
  exit 1;
fi

if [ "$(wc -l <<< $ninty_percent)" -lt "$(wc -l <<< $fifty_percent)" ]; then
  echo "Test failed for downsampling to 50%"
  exit 1;
fi

if [ "$(wc -l <<< $fifty_percent)" -lt "$(wc -l <<< $ten_percent)" ]; then
  echo "Test failed for downsampling to 10%"
  exit 1;
fi

if [ "$(wc -l <<< $ten_percent)" -eq 0 ]; then
  echo "Test failed for downsampling to 10%: I have zero reads"
  exit 1;
fi

echo "$0 passed!";
