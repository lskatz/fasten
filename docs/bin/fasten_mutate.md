# fasten_mutate

## Usage


    Mutates reads. There is no mutation model; only randomness.
    Usage: ../../target/release/fasten_mutate [-h] [-n INT] [-p] [-v] [-s INT] [-m]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
        -s, --snps INT      Number of SNPs (point mutations) to include per read.
        -m, --mark          lowercase all reads but uppercase the SNPs (not yet
                            implemented)
    
