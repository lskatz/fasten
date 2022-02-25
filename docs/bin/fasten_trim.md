# fasten_trim

## Usage


    Blunt-end trims using 0-based coordinates
    Usage: ../../target/release/fasten_trim [-h] [-n INT] [-p] [-v] [-f INT] [-l INT]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
        -f, --first-base INT
                            The first base to keep (default: 0)
        -l, --last-base INT The last base to keep. If negative, counts from the
                            right. (default: 0)
    
