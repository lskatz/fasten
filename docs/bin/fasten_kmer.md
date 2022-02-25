# fasten_kmer

Counts kmers.
Each line is a kmer with two columns separated by tab: kmer, count

## Example

```
cat file.fastq | fasten_kmer -k 15 > 15mers.tsv
```

## Usage


    Usage: ../../target/release/fasten_kmer [-h] [-n INT] [-p] [-v] [-k INT]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
        -k, --kmer-length INT
                            The size of the kmer
    
