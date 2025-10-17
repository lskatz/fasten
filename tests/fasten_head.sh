#!/bin/bash

set -e

THIS_SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
THIS_SCRIPT_NAME=$(basename "$0" | sed 's/\.sh//g')
source "${THIS_SCRIPT_DIR}/test_functions.sh"

INPUT=testdata/four_reads.pe.fastq

reads=$(for i in $(seq 1 1000); do cat $INPUT; done;)
not_downsampled=$(echo "$reads" | ./target/debug/fasten_sample)
first_ten_reads=$(echo "$reads" | head -n 40)

fasten_head=$(echo "$reads" | ./target/debug/fasten_head --reads 10)

if [ "$fasten_head" != "$first_ten_reads" ]; then
  echo "fasten_head does not match head -n 40"
  exit 1
fi

echo "$0 passed!";
