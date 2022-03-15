# fasten_sort

Sort a fastq file.
If the reads are paired end, then the sorted field 
concatenates R1 and R2 before comparisons in the sort.
R1 and R2 reads will stay together if paired end.

Sorting by GC content will give better compression by magic of gzip
and other algorithms.

Sorting can also aid in stable hashsums.

## Examples

```
# stable hashsum
cat file.fastq | fasten_sort | md5sum > file.fastq.md5
# better compression by sorting by GC content
zcat file.fastq.gz | fasten_sort --sort-by GC | gzip -c > smaller.fastq.gz

# get good compression from paired end reads
zcat R1.fastq.gz R2.fastq.gz | fasten_shuffle | \
  fasten_sort --paired-end --sort-by GC | \
  fasten_shuffle -d -1 sorted_1.fastq -2 sorted_2.fastq && \
  gzip -v sorted_1.fastq sorted_2.fastq
# Compare compression between unsorted and sorted
ls -lh sorted_1.fastq.gz sorted_2.fastq.gz
```

## Usage 

    Usage: fasten_sort [-h] [-n INT] [-p] [-v] [-s STRING] [-r]

    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
        -s, --sort-by STRING
                            Sort by either SEQ, GC, or ID. If GC, then the entries
                            are sorted by GC percentage. SEQ and ID are
                            alphabetically sorted.
        -r, --reverse       Reverse sort

