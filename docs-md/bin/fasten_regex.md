# fasten_regex

Filter reads based on a regular expression.

## Examples

```
# Find a specific read
cat file.fastq | fasten_regex --which ID --regex 'my-specific-read-id-1234' > my_read.fastq
# Find a specific read but also keep its pair
cat file.fastq | fasten_regex --which ID --regex 'my-specific-read-id-1234' --paired-end > my_pairs.fastq
# Find a specific motif
cat file.fastq | fasten_regex --which SEQ --regex ATAT > atat-motif.fastq
```

## Usage

    Usage: fasten_regex [-h] [-n INT] [-p] [-v] [-r STRING] [-w String]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
        -r, --regex STRING  Regular expression (default: '.')
        -w, --which String  Which field to match on? ID, SEQ, QUAL. Default: SEQ
    
