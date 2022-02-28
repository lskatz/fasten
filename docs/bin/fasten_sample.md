# fasten_sample

downsample your reads

## Examples

```
# Get 10% of the reads
cat file.fastq | fasten_sample --frequency 0.1 > out.fastq
```

## Usage

    Usage: ../../target/release/fasten_sample [-h] [-n INT] [-p] [-v] [-f FLOAT]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
        -f, --frequency FLOAT
                            Frequency of sequences to print, 0 to 1. Default: 1
    
