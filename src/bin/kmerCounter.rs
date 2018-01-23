extern crate bio;
use bio::io::fastq;
use std::str;
use std::collections::HashMap;

fn main() {
  let reader = fastq::Reader::from_file("reads.fastq").unwrap();
  let kmerlength = 21;

  // instantiate the kmer hash
  let mut kmer_hash: HashMap<String, u64>  = HashMap::new();

  let mut _record_counter=0;
  for record in reader.records(){
      _record_counter+=1;
      let entry = record.unwrap();
      let seq = str::from_utf8(&entry.seq()).unwrap();
      let seq_string = seq.to_string();

      // If the kmer key does not exist yet, then initialize
      // it with 0.
      let mut i = 0;
      loop {
          let last_byte = i+kmerlength;
          if last_byte > seq_string.len() {
              break;
          }
          let kmer_str = &seq_string[i..(i+kmerlength)];
          let count = kmer_hash.entry(String::from(kmer_str)).or_insert(0);
          *count += 1;

          i+=1;
      }
      
      /*
      if record_counter > 30 {
          println!("{:?}", kmer_hash);
          break;
      }
      */
  }

  for (kmer, count) in kmer_hash.iter() {
      println!("{}\t{}",kmer,count);
  }
}
