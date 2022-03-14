# fasten_replace

Streaming editor for fastq data using a find/replace.

## Examples

```
# Force a motif to be lowercase
cat file.fastq | fasten_replace --which SEQ --find ATAT --replace atat > file.fastq
```

## Usage

    Usage: fasten_replace [-h] [-n INT] [-p] [-v] [-f STRING] [-r STRING] [-w STRING]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
        -f, --find STRING   Regular expression (default: '.')
        -r, --replace STRING
                            String to replace each match
        -w, --which STRING  Which field to match on? ID, SEQ, QUAL. Default: SEQ
    
