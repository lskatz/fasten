# fasten_progress

Prints a progress meter for number of fastq entries to stderr.

## Examples

```
# While getting read metrics for a large fastq file, print the progress
# to make the wait a little easier
cat large.fastq | fasten_progress | fasten_metrics
# While shuffling a large fastq file, print the progress
cat large_1.fastq large_2.fastq | fasten_progress | fasten_shuffle > interleaved.fastq
```

## Usage

    Usage: fasten_progress [-h] [-n INT] [-p] [-v] [--id STRING] [--update-every INT]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
            --id STRING     Progress identifier. Default: unnamed
            --update-every INT
                            Update progress every n reads.
    
