# fasten_straighten

Convert a fastq file to a standard 4-lines-per-entry format

## Examples

```
cat weird.fastq | fasten_straighten > four-per-entry.fastq
```

## Usage

    Usage: fasten_straighten [-h] [-n INT] [-p] [-v]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
    
