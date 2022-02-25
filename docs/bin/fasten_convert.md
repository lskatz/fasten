# fasten_convert

Converts file formats to/from different formats.

## Examples

```
# Simple conversion
cat file.fastq | fasten_convert -i fastq -o fasta > out.fasta
# Convert to sam and then to bam
cat file.fastq | fasten_convert -i fastq -o sam   | samtools view -bS > file.bam
# Convert to fastq and then clean
cat file.fasta | fasten_convert -i fasta -o fastq | fasten_clean > cleaned.fastq
```


## Usage


    Usage: ../../target/release/fasten_convert [-h] [-n INT] [-p] [-v] [-i FORMAT] [-o FORMAT]
    
    Options:
        -h, --help          Print this help menu.
        -n, --numcpus INT   Number of CPUs (default: 1)
        -p, --paired-end    The input reads are interleaved paired-end
        -v, --verbose       Print more status messages
        -i, --in-format FORMAT
                            The input format for stdin
        -o, --out-format FORMAT
                            The output format for stdin
      FORMAT can be: fastq, fasta, sam
    
