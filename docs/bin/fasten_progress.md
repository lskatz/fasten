# fasten_progress

## Usage


    Prints a progress meter for number of fastq entries.
    Usage: ../../target/release/fasten_progress [-h] [-n INT] [-p] [-v] [--id STRING] [--update-every INT]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
            --id STRING     Progress identifier. Default: unnamed
            --update-every INT
                            Update progress every n reads.
    
