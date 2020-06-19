#!/bin/bash

NUMCPUS=1

if [ "$1" == "" ]; then
  echo "Determines read metrics on a set of fastq files."
  echo "Usage: $0 *.fastq[.gz]" >&2
  exit 1
fi

which fasten_metrics >& /dev/null
if [ $? -gt 0 ]; then
  echo "ERROR: could not find fasten_metrics in the path" >&2
  exit 1
fi

set -e

echo -e "file\ttotalLength\tnumReads\tavgReadLength\tavgQual"
echo "$@" | xargs -P $NUMCPUS -n 1 bash -c '
  if [ ! -e $0 ]; then
    echo "ERROR: could not find $0" >&2
    exit 1
  fi

  extension="${0##*.}"
  metrics="\n"
  if [ "$extension" == "gz" ]; then
    metrics=$(zcat $0 | fasten_metrics | tail -n +2)
  else
    metrics=$(fasten_metrics < $0)
  fi
  echo -e "$0\t$metrics"
'

