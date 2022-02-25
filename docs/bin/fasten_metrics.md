# fasten_metrics

## Usage


    Gives read metrics on a read set.
    Usage: ../../target/release/fasten_metrics [-h] [-n INT] [-p] [-v] [--each-read] [--distribution STRING]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
            --each-read     Print the metrics for each read. This creates very
                            large output
            --distribution STRING
                            Print the distribution for each metric. Must supply
                            either 'normal' or 'nonparametric'
    
