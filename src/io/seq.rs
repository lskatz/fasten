use std::collections::HashMap;

#[test]
/// Test whether a cleanable sequence is instantiated
fn test_new_seq() {
    let id   = "MY_ID".to_string();
    let seq  = "AATNGGCC".to_string();
    let qual = "#ABCDE!!".to_string();
    let cleanable = Seq::new(&id,&seq,&qual);
    
    let formatted = format!("@{}\n{}\n+\n{}", &id, &seq, &qual);
    assert_eq!(cleanable.to_string(), formatted);
}
#[test]
/// Test whether a cleanable sequence can be cleaned
fn test_cleanable() {
    let id   = "MY_ID".to_string();
    let seq  = "AATNGGCC".to_string();
    let qual = "#ABCDE!!".to_string();
    let mut cleanable = Seq::new(&id,&seq,&qual);
    
    cleanable.lower_ambiguity_q();
    cleanable.trim();
    assert_eq!(cleanable.to_string(), "@MY_ID\nATNGG\n+\nAB!DE".to_string());
}


/// A sequence struct that contains the ID, sequence, and quality cigar line
pub struct Seq {
  pub id:   String,
  pub seq:  String,
  pub qual: String,
  pub thresholds: HashMap<String,f32>,
}

/// A sequence that can be cleaned
pub trait Cleanable{
    /// new sequence object
    fn new (id: &String, seq: &String, qual: &String) -> Seq;
    /// Make a blank sequence object.
    fn blank () -> Seq;
    /// Determine if it is a blank sequence.
    fn is_blank (&self) -> bool;
    /// Make a seq object from a String.
    fn from_String (seq_str: &String) -> Seq;
    /// sanitize an identifier string
    fn sanitize_id(id: &String) -> (String);
    /// lower any low quality base to a zero and "N"
    fn lower_ambiguity_q(&mut self) -> ();
    /// Trim sequences based on quality
    fn trim(&mut self)  -> ();
    /// Reports bool whether the read passes thresholds.
    fn is_high_quality(&mut self) -> bool; 
    /// Make a String object
    fn to_string(&self) -> String;
    /// Print the result of to_string()
    fn print(&self)     -> ();
}

impl Cleanable for Seq {
    /// Make a new cleanable sequence object
    fn new (id: &String, seq: &String, qual: &String) -> Seq{
        let id_copy = Self::sanitize_id(&id);
        let mut thresholds = HashMap::new();
        thresholds.insert("min_avg_qual".to_string(),20.0);
        thresholds.insert("min_length".to_string(),100.0);
        thresholds.insert("min_trim_qual".to_string(),20.0);

        return Seq{
            id:   id_copy,
            seq:  seq.clone(),
            qual: qual.clone(),
            thresholds: thresholds,
        };
    }
    /// Make a blank sequence object.
    fn blank () -> Seq{
        return Seq::new(&String::new(),&String::new(),&String::new());
    }
    /// Determine if it is a blank sequence.
    fn is_blank (&self) -> bool {
        if self.seq.len() == 0 && self.qual.len() == 0 {
            return true;
        }
        return false;
    }
    /// Create a sequence object from a string.
    /// TODO make it more like the careful method than quick.
    fn from_String (seq_str: &String) -> Seq {
        let mut lines = seq_str.lines();
        let id = lines.next().expect("Could not parse ID");
        let seq = lines.next().expect("Could not parse sequence");
        lines.next().expect("Could not parse +");
        let qual_opt = lines.next();
        if qual_opt == None {
            return Seq::blank();
        }
        let qual = qual_opt.expect("Could not read the qual line");

        return Seq{
            id:    id.to_string(),
            seq:   seq.to_string(),
            qual:  qual.to_string(),
            thresholds: HashMap::new(),
        }
    }

    /// Read an identifier and return a cleaned version,
    /// e.g., removing @ in a fastq identifier.
    fn sanitize_id(id: &String) -> (String) {
        if id.len() == 0 {
            return String::new();
        }
        let mut id_copy = id.clone();
        if id_copy.chars().nth(0).expect("ID was empty") == '@' {
            id_copy.pop();
        }
        return id_copy;
    }
        

    /// Alter any ambiguity site with a quality=0
    fn lower_ambiguity_q(&mut self){
        let zero_score:char  = 33 as char;
        let low_score :char  = (33 + 20) as u8 as char;

        let mut low_qual_idx = vec![false; self.seq.len()];
        // First find the indices b/c it is so slow to
        // edit a string in-place one char at a time.
        for (i,nt) in self.seq.chars().enumerate(){
            if nt == 'N' || nt == 'n' || self.qual.chars().nth(i).expect("Expected a char") < low_score {
                low_qual_idx[i] = true;
            }
        }
        
        let mut new_seq =String::new();
        let mut new_qual=String::new();
        for (i,nt) in self.seq.chars().enumerate(){
            if low_qual_idx[i] {
                new_seq.push('N');
                new_qual.push(zero_score);
            } else{
                new_seq.push(nt);
                new_qual.push_str(&self.qual[i..i+1]);
            }
        }

        self.seq=new_seq;
        self.qual=new_qual;
    }

    /// Trim the ends of reads with low quality
    fn trim(&mut self) {
        let min_qual = 20;

        let mut trim5=0;
        let mut trim3=&self.qual.len()-0;
        
        // 5'
        for qual in self.qual.chars(){
            if qual as u8 - 33 < min_qual {
                trim5+=1;
            } else {
                break;
            }
        }

        // 3'
        for qual in self.qual.chars().rev() {
            if qual as u8 - 33 < min_qual {
                trim3-=1;
            } else {
                break;
            }
        }
        
        if trim5 >= trim3 {
            self.qual = String::new();
            self.seq  = String::new();
        } else {
            self.qual = self.qual[trim5..trim3].to_string();
            self.seq  = self.seq[trim5..trim3].to_string();
        }
    }

    /// Reports bool whether the read passes thresholds.
    fn is_high_quality(&mut self) -> bool {
        // fail if seq len is short
        let min_length = self.thresholds.get(&"min_length".to_string()).expect("min_length does not look like a number");
        let seq_len = self.seq.len() as f32;
        if seq_len < *min_length {
            //.parse::<i32>().unwrap();
            return false;
        }

        let mut total_qual = 0;
        for qual in self.qual.chars() {
            total_qual += qual as u32;
        }

        // fail if qual is low
        let avg_qual = (total_qual as f32/seq_len) - 33.0;
        let min_qual = self.thresholds.get(&"min_avg_qual".to_string()).expect("min_avg_qual does not look like a number");
        if avg_qual < *min_qual {
            return false;
        }

        return true;
    }

    fn to_string(&self) -> String {
        let mut entry = String::new();
        if self.id.len() > 0 && self.id.chars().nth(0).expect("Seq ID was not set") != '@' {
            entry.push('@');
        }
        entry.push_str(self.id.trim());
        entry.push_str("\n");
        entry.push_str(self.seq.trim());
        entry.push_str("\n+\n");
        entry.push_str(&self.qual.trim());
        return entry;
    }
    fn print(&self) -> () {
        println!("{}",self.to_string());
    }
} 


