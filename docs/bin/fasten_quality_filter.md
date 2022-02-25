# fasten_quality_filter

## Usage


    Transforms any low-quality base to 'N'
    Usage: ../../target/release/fasten_quality_filter [-h] [-n INT] [-p] [-v] [-m INT]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
        -m, --max-quality INT
                            The maximum quality at which a base will be
                            transformed to 'N'
    
