# fasten_shuffle

Interleaves reads from either stdin or file parameters.

Many fasten executables are aware of paired end reads
but they need to be in interleaved format.
This script transforms R1 and R2 reads into interleaved format.

## Examples

### Shuffling

```
# Simple transformation of R1 and R2 into interleaved
cat file_1.fastq file_2.fastq | fasten_shuffle > interleaved.fastq
fasten_shuffle -1 file_1.fastq -2 file_2.fastq > interleaved.fastq
# interleave R1 and R2 and pipe it into another executable with --paired-end
cat file_1.fastq file_2.fastq | fasten_randomize --paired-end | head -n 8 > random-pair.fastq
# ... or to another executable with --paired-end
cat file_1.fastq file_2.fastq | fasten_sample --paired-end --frequency 0.2 > downsample.20percent.fastq
```

### Deshuffling

```
cat interleaved.fastq | fasten_shuffle -d -1 1.fastq -2 2.fastq
```

## Usage

    Usage: fasten_shuffle [-h] [-n INT] [-p] [-v] [-d] [-1 1.fastq] [-2 2.fastq]
    
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
    
