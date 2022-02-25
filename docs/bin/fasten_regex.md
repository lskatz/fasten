# fasten_regex

## Usage


    Filter reads based on a regular expression.
    Usage: ../../target/release/fasten_regex [-h] [-n INT] [-p] [-v] [-r STRING] [-w String]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
        -r, --regex STRING  Regular expression (default: '.')
        -w, --which String  Which field to match on? ID, SEQ, QUAL. Default: SEQ
    
