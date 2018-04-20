#!/bin/bash

set -e

replace="X"
# https://stackoverflow.com/a/5349842
hundredX=$(printf 'X%.0s' {1..100})


reads=$(cat testdata/four_reads.pe.fastq)
default=$(echo "$reads" | target/debug/fasten_replace --replace $replace)
replace_ids=$(echo "$reads" | target/debug/fasten_replace --which ID --find r --replace $replace)
replace_seqs=$(echo "$reads" | target/debug/fasten_replace --which SEQ --find TTTT --replace $replace)
replace_quals=$(echo "$reads" | target/debug/fasten_replace --which QUAL --find '\*4' --replace $replace)

# Count how many times X appears which is the thing with which we replaced
if [ "$(grep -c $hundredX <<< "$default")" -ne 8 ]; then
  echo "ERROR with default arguments and replace string $replace"
  exit 1
fi
if [ "$(grep -c "Xead" <<< "$replace_ids")" -ne 8 ]; then
  echo "ERROR with replacing IDs and replace string $replace"
  echo "$replace_ids"
  exit 1
fi
if [ "$(grep -c X <<< "$replace_seqs")" -ne 3 ]; then
  echo "ERROR with replacing SEQs and replace string $replace"
  echo "$replace_seqs"
  exit 1
fi
if [ "$(grep -c X <<< "$replace_quals")" -ne 4 ]; then
  echo "ERROR with replacing QUALs and replace string $replace"
  echo "$replace_quals"
  exit 1
fi

echo "$0 passed!"

