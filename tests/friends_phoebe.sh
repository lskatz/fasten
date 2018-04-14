#!/bin/bash

set -e

INPUT=testdata/four_reads.pe.fastq;

lots_of_reads=$(for i in $(seq 1 100); do cat $INPUT; done;);

# Randomize reads
random_reads=$(echo "$lots_of_reads" | ./target/debug/friends_phoebe)
random_pe_reads=$(echo "$lots_of_reads" | ./target/debug/friends_phoebe --paired-end)

# Test sorted output
random_sorted=$(echo "$random_reads" | sort)
random_pe_sorted=$(echo "$random_pe_reads" | sort)
reads_sorted=$(echo "$lots_of_reads" | sort)
if [ "$random_sorted" != "$reads_sorted" ]; then
  echo "ERROR: did not get the same reads out as I fed them to the random reads script"
  exit 1
fi
if [ "$random_pe_sorted" != "$reads_sorted" ]; then
  echo "ERROR: did not get the same PE reads out as I fed them to the random reads script"
  exit 1
fi

# Test to see if the random reads are not the same as the 
# input reads.
# It's possible to have the same random reads
# as input, and so unfortunately there is a chance that
# this test will fail even though the output is valid.
# The chances of that happening without paired end is 1/100!, 9e-157
if [ "$random" == "$(cat $INPUT)" ]; then
  echo "WARNING: random reads look like they are in the same order as the original file"
  echo "If you think that you got this result by chance, please run this test one more time."
  exit 1
fi

# Test to see if PE reads stayed PE
echo "$random_pe_reads" | ./target/debug/friends_ung

echo "$0 passed!"

