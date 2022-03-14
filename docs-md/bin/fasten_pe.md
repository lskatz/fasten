# fasten_pe

Determine paired-end-ness in an interleaved file.
Exit code of 0 indicates PE. Exit code > 0 indicates single end.

## Examples

```
# Test the file and then print a message with the exit code
cat file.fastq | fasten_pe; echo "Reads were paired-end? $?";
# Test the file and if it is paired end (exit code 0), then print a message
cat file.fastq | fasten_pe || echo "Reads were paired end.";
```

## Usage

    Usage: fasten_pe [-h] [-n INT] [-p] [-v] [--print-reads]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
            --print-reads   Print each read. Useful for Unix pipes.
    
