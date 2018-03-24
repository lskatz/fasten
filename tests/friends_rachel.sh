#!/bin/bash

set -e

avg_qual=$(./target/debug/friends_rachel < testdata/four_reads.pe.fastq | cut -f 4 |tail -n 1)
if [ "$avg_qual" != "19.53875" ]; then
  exit 1;
fi

echo "friends_rachel passed"

