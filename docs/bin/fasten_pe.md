# fasten_pe

## Usage


    Determine paired-end-ness in an interleaved file. Exit code of 0 indicates PE. Exit code > 0 indicates SE.
    Usage: ../../target/release/fasten_pe [-h] [-n INT] [-p] [-v] [--print-reads]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
            --print-reads   Print each read. Useful for Unix pipes.
    
