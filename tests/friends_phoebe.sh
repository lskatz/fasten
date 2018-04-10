#!/bin/bash

set -e

INPUT=testdata/four_reads.pe.fastq;

# It's tough to test random reads, so let's just 
# randomize and then test sorted output
random=$(cat $INPUT | ./target/debug/friends_phoebe)
random_sorted=$(echo "$random" | sort)
INPUT_sorted=$(cat "$INPUT" | sort)
if [ "$random_sorted" != "$INPUT_sorted" ]; then
  echo "ERROR: did not get the same reads out as I fed them to the random reads script"
  exit 1
fi

# Because it's possible to have the same random reads
# as input, we simply give a warning if it's the same.
# The chances of that happening without paired end is 1/8!, 1/40320
if [ "$random" == "$(cat $INPUT)" ]; then
  echo "WARNING: random reads look like they are in the same order as the original file"
  echo "The chances of that happening are 1/40320. Weird."
fi

echo "$0 passed!"

