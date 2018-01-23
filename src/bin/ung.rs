extern crate bio;
use bio::io::fastq;
use std::io;

fn main() {
  let reader = fastq::Reader::new(io::stdin());
  let mut writer = fastq::Writer::new(io::stdout());

  let mut _record_counter=0;
  for record in reader.records(){
      _record_counter+=1;
      let fastq_entry=record.unwrap();
      writer.write_record(&fastq_entry).unwrap();
  }
  writer.flush().unwrap();
}
