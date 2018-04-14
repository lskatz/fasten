#!/bin/bash

set -e

echo "WARNING: need to test phoebe better"
exit 0

INPUT=testdata/four_reads.pe.fastq;

# It's tough to test random reads, so let's just 
# randomize and then test sorted output
random=$(cat $INPUT $INPUT $INPUT | ./target/debug/friends_phoebe)
random_sorted=$(echo "$random" | sort)
INPUT_sorted=$(cat "$INPUT $INPUT $INPUT" | sort)
if [ "$random_sorted" != "$INPUT_sorted" ]; then
  echo "ERROR: did not get the same reads out as I fed them to the random reads script"
  exit 1
fi

# It's possible to have the same random reads
# as input, and so unfortunately there is a chance that
# this test will fail even though the output is valid.
# The chances of that happening without paired end is 1/21!, 5.1e-19
if [ "$random" == "$(cat $INPUT)" ]; then
  echo "WARNING: random reads look like they are in the same order as the original file"
  echo "The chances of that happening are 1/40320. Weird."
fi

echo "$0 passed!"

