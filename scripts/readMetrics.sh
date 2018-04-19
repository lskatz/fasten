#!/bin/bash

NUMCPUS=1

which friends_rachel >& /dev/null
if [ $? -gt 0 ]; then
  echo "ERROR: could not find friends_rachel in the path" >&2
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
    metrics=$(zcat $0 | friends_rachel | tail -n +2)
  else
    metrics=$(friends_rachel < $0)
  fi
  echo -e "$0\t$metrics"
'

