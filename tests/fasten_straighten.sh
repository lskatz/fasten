#!/bin/bash

set -e

INPUT=testdata/four_reads.gt_16_lines.fastq
CORRECT=testdata/four_reads.fastq

corrected=$(./target/debug/fasten_straighten < testdata/four_reads.gt_16_lines.fastq)

if [ "$corrected" != "$(cat $CORRECT)" ]; then
  echo "fasten_straighten did not correct the reads"
  exit 1
fi

echo "Fasten_straighten passed!"
