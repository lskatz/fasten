# fasten_clean

Trim and filter reads

## Examples

```
cat file.fastq | fasten_clean --min-avg-quality 25 --min-trim-quality 25 > out.fastq
```

## Usage

    Usage: fasten_clean [-h] [-n INT] [-p] [-v] [--min-length INT] [--min-avg-quality FLOAT] [--min-trim-quality INT]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
            --min-length INT
                            Minimum length for each read in bp
            --min-avg-quality FLOAT
                            Minimum average quality for each read
            --min-trim-quality INT
                            Trim the edges of each read until a nucleotide of at
                            least X quality is found
    
