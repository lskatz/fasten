#!/bin/bash

NUMCPUS=4

if [ "$3" == "" ]; then
  echo "Sorts and compresses paired end reads"
  echo "Usage: $0 outdir/ something_[12].fastq.gz [another_[12].fastq.gz...]" >&2
  echo "  where the reads are paired according to the order on ARGV"
  exit 1
fi

which fasten_sort >& /dev/null
if [ $? -gt 0 ]; then
  echo "ERROR: could not find fasten_sort in the path" >&2
  exit 1
fi

set -e

export outdir=$1
shift

if [ ! -d $outdir ]; then
  echo "ERROR: could not find dir $outdir"
  exit 1
fi

echo "$@" | xargs -P $NUMCPUS -n 2 bash -c '
  R1=$0
  R2=$1
  if [ ! -e $R1 ]; then 
    echo "Could not find $R1; not sorting." >&2
    exit 0;
  fi
  if [ ! -e $R2 ]; then 
    echo "Could not find $R2; not sorting." >&2
    exit 0;
  fi
  echo "sorting $R1 and $R2" >&2

  out_R1=$outdir/$(basename $R1)
  out_R2=$outdir/$(basename $R2)
  out_R1_ungz=$outdir/$(basename $R1 .gz)
  out_R2_ungz=$outdir/$(basename $R2 .gz)

  if [ -e $out_R1 ]; then 
    echo "  Found $out_R1; not overwriting" >&2
    exit 0;
  fi
  if [ -e $out_R2 ]; then 
    echo "  Found $out_R2; not overwriting" >&2
    exit 0;
  fi
  name=$(basename $R1 .fastq.gz)
#    fasten_progress --update-every 100000 --print --id zcat_pairs_$name | \
  zcat $R1 $R2 | \
    fasten_shuffle | \
    fasten_sort --paired-end --sort-by SEQ | \
    fasten_shuffle -1 $out_R1_ungz -2 $out_R2_ungz -d
  
  echo "compressing $out_R1_ungz $out_R2_ungz" >&2
  gzip -9f $out_R1_ungz $out_R2_ungz
  
  echo "Created $out_R1 $out_R2" >&2
'

