var searchIndex = JSON.parse('{\
"fasten":{"doc":"Perform random operations on fastq files, using unix …","t":[5,5,0,5,14,0,0,3,11,11,11,11,11,11,11,11,11,11,11,8,3,10,11,11,11,11,11,11,10,11,12,11,10,11,10,11,10,11,10,11,12,10,11,12,10,11,12,12,11,10,11,10,11,11,11,11],"n":["eexit","fasten_base_options","io","logmsg","print","fastq","seq","FastqReader","borrow","borrow_mut","from","into","into_iter","new","new_careful","next","try_from","try_into","type_id","Cleanable","Seq","blank","blank","borrow","borrow_mut","clone","clone_into","from","from_string","from_string","id","into","is_blank","is_blank","is_high_quality","is_high_quality","lower_ambiguity_q","lower_ambiguity_q","new","new","pairid","print","print","qual","sanitize_id","sanitize_id","seq","thresholds","to_owned","to_string","to_string","trim","trim","try_from","try_into","type_id"],"q":["fasten","","","","","fasten::io","","fasten::io::fastq","","","","","","","","","","","","fasten::io::seq","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Propagate an error by printing invalid read(s)","a function that reads an options object and adds fasten …","","Print a formatted message to stderr ","Rewrite print!() so that it doesn’t panic on broken pipe.","","","A FastQ reader","","","","","","","","There are two flavors of next: either read  quickly or …","","","","A sequence that can be cleaned","A sequence struct that contains the ID, sequence, and …","Make a blank sequence object.","Make a blank sequence object.","","","","","","Make a seq object from a String.","Create a sequence object from a string. TODO make it more …","","","Determine if it is a blank sequence.","Determine if it is a blank sequence.","Reports bool whether the read passes thresholds.","Reports bool whether the read passes thresholds.","lower any low quality base to a zero and “N”","Alter any ambiguity site with a quality=0","new sequence object","Make a new cleanable sequence object","","Print the result of to_string()","","","sanitize an identifier string","Read an identifier and return a cleaned version, e.g., …","","","","Make a String object","","Trim sequences based on quality","Trim the ends of reads with low quality","","",""],"i":[0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,0,0,2,3,3,3,3,3,3,2,3,3,3,2,3,2,3,2,3,2,3,3,2,3,3,2,3,3,3,3,2,3,2,3,3,3,3],"f":[[[]],[[],["options",3]],null,[[["asref",8,[["str",15]]]]],null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[],["fastqreader",3]],[[],["fastqreader",3]],[[],["option",4,[["seq",3]]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,[[],["seq",3]],[[],["seq",3]],[[]],[[]],[[],["seq",3]],[[]],[[]],[[["string",3]],["seq",3]],[[["string",3]],["seq",3]],null,[[]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[]],[[]],[[["string",3],["string",3],["string",3]],["seq",3]],[[["string",3],["string",3],["string",3]],["seq",3]],null,[[]],[[]],null,[[["string",3]],["string",3]],[[["string",3]],["string",3]],null,null,[[]],[[],["string",3]],[[],["string",3]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]]],"p":[[3,"FastqReader"],[8,"Cleanable"],[3,"Seq"]]},\
"fasten_clean":{"doc":"Trim and filter reads","t":[5,5,5,5],"n":["avg_quality","clean_entry","main","trim"],"q":["fasten_clean","","",""],"d":["Determine average quality of a qual cigar string, e.g., …","Cleans a SE or PE read","","Trim the ends of reads with low quality"],"i":[0,0,0,0],"f":[[[["string",3]],["f32",15]],[[["vec",3,[["string",3]]],["usize",15],["f32",15],["u8",15],["u8",15],["sender",3,[["string",3]]],["sender",3,[["string",3]]]]],[[]],[[["string",3],["string",3],["u8",15]]]],"p":[]},\
"fasten_combine":{"doc":"Collapse identical reads into single reads, recalculating …","t":[17,17,5,5],"n":["READ_SEPARATOR","TEN","combine_error_vectors","main"],"q":["fasten_combine","","",""],"d":["Glues together paired end reads internally and is a …","need this constant because the compiler had a problem with …","Combines vectors of error probabilities such that the rate …",""],"i":[0,0,0,0],"f":[null,null,[[["vec",3],["vec",3]],["vec",3,[["f32",15]]]],[[]]],"p":[]},\
"fasten_convert":{"doc":"Convert between different sequence formats","t":[3,11,11,11,11,11,11,11,11,11,12,12,11,5,11,12,12,5,5,5,12,12,11,11,11,11,5,5,5],"n":["FastenSeq","as_fasta","as_fastq","as_sam","borrow","borrow_mut","clone","clone_into","fmt","from","id1","id2","into","main","new","qual1","qual2","read_fasta","read_fastq","read_sam","seq1","seq2","to_owned","try_from","try_into","type_id","write_fasta","write_fastq","write_sam"],"q":["fasten_convert","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Struct that can handle paired end reads","Return a formatted string as a fasta entry","Return a formatted string as a fastq entry","Return a formatted string as a sam entry","","","","","","","","","","","a blank new object is a set of blank strings for each value","","","Read fasta from stdin and transmit it to a channel","Read fastq from stdin and transmit it to a channel","Read sam from stdin and transmit it to a channel","","","","","","","Read from a channel and print as fasta","Read from a channel and print as fastq","Read from a channel and print as sam"],"i":[0,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,0,0,0,1,1,1,1,1,1,0,0,0],"f":[null,[[],["string",3]],[[],["string",3]],[[],["string",3]],[[]],[[]],[[],["fastenseq",3]],[[]],[[["formatter",3]],["result",6]],[[]],null,null,[[]],[[]],[[],["fastenseq",3]],null,null,[[["sender",3,[["fastenseq",3]]],["bool",15]]],[[["sender",3,[["fastenseq",3]]],["bool",15]]],[[["sender",3,[["fastenseq",3]]],["bool",15]]],null,null,[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[["receiver",3,[["fastenseq",3]]]]],[[["receiver",3,[["fastenseq",3]]]]],[[["receiver",3,[["fastenseq",3]]]]]],"p":[[3,"FastenSeq"]]},\
"fasten_kmer":{"doc":"","t":[5,5,5,5,5],"n":["count_kmers","kmers_in_str","main","revcomp","switch_base"],"q":["fasten_kmer","","","",""],"d":["","","","reverse-complement a dna sequence","Complementary nucleotide"],"i":[0,0,0,0,0],"f":[[[["stdin",3],["usize",15],["bool",15]]],[[["str",15],["usize",15],["bool",15]],["hashmap",3,[["string",3],["u32",15]]]],[[]],[[["str",15]],["string",3]],[[["char",15]],["char",15]]],"p":[]},\
"fasten_metrics":{"doc":"","t":[5,5,5],"n":["average_quality","main","standard_deviation"],"q":["fasten_metrics","",""],"d":["given a cigar line for quality, return its average","",""],"i":[0,0,0],"f":[[[["str",15]],["f32",15]],[[]],[[["vec",3]],["f32",15]]],"p":[]},\
"fasten_mutate":{"doc":"","t":[5,5],"n":["main","mutate"],"q":["fasten_mutate",""],"d":["",""],"i":[0,0],"f":[[[]],[[["str",15],["vec",3],["u8",15],["bool",15]],["string",3]]],"p":[]},\
"fasten_pe":{"doc":"","t":[5,5,5],"n":["is_paired_end_miseq","is_paired_end_slash12","main"],"q":["fasten_pe","",""],"d":["","",""],"i":[0,0,0],"f":[[[["vec",3],["vec",3]],["u8",15]],[[["vec",3],["vec",3]],["u8",15]],[[]]],"p":[]},\
"fasten_progress":{"doc":"","t":[5],"n":["main"],"q":["fasten_progress"],"d":[""],"i":[0],"f":[[[]]],"p":[]},\
"fasten_quality_filter":{"doc":"","t":[5],"n":["main"],"q":["fasten_quality_filter"],"d":[""],"i":[0],"f":[[[]]],"p":[]},\
"fasten_randomize":{"doc":"","t":[5,5],"n":["main","print_reads_from_stdin"],"q":["fasten_randomize",""],"d":["",""],"i":[0,0],"f":[[[]],[[["u32",15]]]],"p":[]},\
"fasten_regex":{"doc":"","t":[5],"n":["main"],"q":["fasten_regex"],"d":[""],"i":[0],"f":[[[]]],"p":[]},\
"fasten_replace":{"doc":"","t":[5],"n":["main"],"q":["fasten_replace"],"d":[""],"i":[0],"f":[[[]]],"p":[]},\
"fasten_sample":{"doc":"","t":[5],"n":["main"],"q":["fasten_sample"],"d":[""],"i":[0],"f":[[[]]],"p":[]},\
"fasten_shuffle":{"doc":"","t":[5,5,5,5],"n":["deshuffle","main","read_seqs","shuffle"],"q":["fasten_shuffle","","",""],"d":["","","",""],"i":[0,0,0,0],"f":[[[["matches",3]]],[[]],[[["string",3]],["vec",3,[["seq",3]]]],[[["matches",3]]]],"p":[]},\
"fasten_sort":{"doc":"","t":[3,11,11,11,11,11,11,12,12,11,5,12,12,12,12,12,5,11,11,11,11],"n":["Seq","borrow","borrow_mut","clone","clone_into","fmt","from","id1","id2","into","main","pe","qual1","qual2","seq1","seq2","sort_entries","to_owned","try_from","try_into","type_id"],"q":["fasten_sort","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","Sort fastq entries in a vector","","","",""],"i":[0,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,0,1,1,1,1],"f":[null,[[]],[[]],[[],["seq",3]],[[]],[[["formatter",3]],["result",6]],[[]],null,null,[[]],[[]],null,null,null,null,null,[[["vec",3,[["seq",3]]],["str",15],["bool",15]],["vec",3,[["seq",3]]]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]]],"p":[[3,"Seq"]]},\
"fasten_straighten":{"doc":"","t":[5],"n":["main"],"q":["fasten_straighten"],"d":[""],"i":[0],"f":[[[]]],"p":[]},\
"fasten_trim":{"doc":"","t":[5,5],"n":["main","trim_worker"],"q":["fasten_trim",""],"d":["",""],"i":[0,0],"f":[[[]],[[["vec",3,[["seq",3]]],["usize",15],["usize",15],["sender",3,[["string",3]]]]]],"p":[]},\
"fasten_validate":{"doc":"","t":[5],"n":["main"],"q":["fasten_validate"],"d":[""],"i":[0],"f":[[[]]],"p":[]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};