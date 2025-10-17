#!/bin/bash

set -e

THIS_SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
THIS_SCRIPT_NAME=$(basename "$0" | sed 's/\.sh//g')
source "${THIS_SCRIPT_DIR}/test_functions.sh"

INPUT=testdata/four_reads.pe.fastq

# Use md5sum to compare
first_two_reads=$(cat $INPUT | head -n 8 )
fasten_head=$(cat $INPUT | ./target/debug/fasten_head --reads 2 )

if [ "$fasten_head" != "$first_two_reads" ]; then
  echo "fasten_head does not match head -n 8"
  exit 1
fi

echo "$0 passed!";
