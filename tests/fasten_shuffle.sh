#!/bin/bash

set -e

PE=testdata/four_reads.pe.fastq;
R1=testdata/R1.fastq
R2=testdata/R2.fastq

interleaved=$(cat $R1 $R2 | ./target/debug/fasten_shuffle)
shouldbe=$(cat $PE)
if [ "$interleaved" != "$shouldbe" ]; then
  echo "ERROR: did not get interleaved reads"
  exit 1
fi

tmpdir=$(mktemp --directory --tmpdir=. FRIENDS_JOEY.XXXXXX)
trap " { rm -rf $tmpdir; } " EXIT

# deshuffling test
shouldbe_R1="$tmpdir/R1.fastq"
shouldbe_R2="$tmpdir/R2.fastq"
cat $PE | ./target/debug/fasten_shuffle --deshuffle -1 $shouldbe_R1 -2 $shouldbe_R2
if [ "$(cat $shouldbe_R1)" != "$(cat $R1)" ]; then
  echo "ERROR: did not deshuffle correctly to R1 (and probably to R2)"
  exit 1
fi
if [ "$(cat $shouldbe_R2)" != "$(cat $R2)" ]; then
  echo "ERROR: did not deshuffle correctly to R2 but somehow made R1 correctly"
  exit 1
fi

echo "$0 passed!"

