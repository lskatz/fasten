# fasten_combine

Collapse identical reads into single reads, recalculating quality values.
If paired end, then each set of reads must be identical to be collapsed.
_Warning_: due to multiple reads collapsing into one, read identifiers will be reconstituted.

## Examples

```
cat file.fastq | fasten_combine > combined.fastq
```

## Usage

    Usage: fasten_combine [-h] [-n INT] [-p] [-v] [--max-qual-char CHAR] [--min-qual-char CHAR]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
            --max-qual-char CHAR
                            Maximum quality character (default: I)
            --min-qual-char CHAR
                            Minimum quality character (default: !)
    
    NOTE: range of quality scores is !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHI
