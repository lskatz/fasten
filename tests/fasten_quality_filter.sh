#!/bin/bash

INPUT="testdata/four_reads.pe.fastq"
reads=$(cat $INPUT)
masked=$(./target/debug/fasten_quality_filter --max-quality 20 < testdata/four_reads.pe.fastq)

num_Ns=$(
  ./target/debug/fasten_quality_filter --max-quality 20 < testdata/four_reads.pe.fastq | \
    perl -lane '
      for $char(split(//,$_)){
        $N++ if($char eq "N");
      } END{print $N}
    '
  );

if [ "$num_Ns" -ne 393 ]; then
  echo "ERROR: number of expected bases were not masked"
  exit 1
fi

echo "$0 passed!"
