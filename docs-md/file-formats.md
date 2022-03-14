# File formats

All scripts in this software package accept fastq files and print fastq files, unless otherwise noted. When the reads are paired end, this software package reads interleaved fastq files and prints them. All input and output files must be uncompressed.

## Interleaved fastq

The interleaved fastq format is where the first read of a pair (R1) is immediately followed by its pair (R2). Normally, R1 and R2 are "split" into separate files. For example using the test data in this repository:

    $ cat testdata/four_reads.pe.fastq | perl -lane 'print if($i++ % 2 == 0);'
    @read0/1
    +
    @read0/2
    +
    @read1/1
    +
    @read1/2
    +
    @read2/1
    +
    @read2/2
    +
    @read3/1
    +
    @read3/2
    +

