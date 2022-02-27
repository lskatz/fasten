# fasten_randomize
    
Create random reads from stdin.

## Examples

```
# General usage to randomize the order of the reads
cat file.fastq | fasten_randomize > random.fastq
# Get one random read. Entries will always be in a 4-line format.
cat file.fastq | fasten_randomize | head -n 4 > one_read.fastq
# keep the paired ends together if paired
cat R1.fastq R2.fastq | fasten_shuffle | fasten_randomize --paired-end | head -n 8 > one_pair.fastq
```

## Usage

    Usage: ../../target/release/fasten_randomize [-h] [-n INT] [-p] [-v]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
    
