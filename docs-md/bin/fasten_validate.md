# fasten_validate

Validates your reads and makes you feel good about yourself!

## Examples

```
# Quick validation with stderr message
cat file.fastq | fasten_validate --verbose
# Validate that your reads are paired end
cat R1.fastq R2.fastq | fasten_shuffle | fasten_validate
```

## Usage

    Usage: fasten_validate [-h] [-n INT] [-p] [-v] [--min-length INT] [--min-quality FLOAT] [--paired-end] [--print-reads] [-v]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
            --min-length INT
                            Minimum read length allowed
            --min-quality FLOAT
                            Minimum quality allowed
            --paired-end    The reads are interleaved paired-end
            --print-reads   Print the reads as they are being validated (useful
                            for unix pipes)
        -v, --verbose       
    
