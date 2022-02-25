# fasten_shuffle

## Usage


    Interleaves reads from either stdin or file parameters.
    Usage: ../../target/release/fasten_shuffle [-h] [-n INT] [-p] [-v] [-d] [-1 1.fastq] [-2 2.fastq]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
        -d, --deshuffle     Deshuffle reads from stdin
        -1 1.fastq          Forward reads. If deshuffling, reads are written to
                            this file.
        -2 2.fastq          Forward reads. If deshuffling, reads are written to
                            this file.
    
