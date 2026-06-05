
enum  CaseStatus {
  Unopened,
  Investigating {
    location:String,
    days_elapsed: u32,
  },
  Interrogating {
    suspect_name:String,
    tension:u8,
  },
  Solved{
    culprit:String
  },
  Failed{
    reason:String
  }
}


#[derive(Debug)]
enum ClueType {
  Physical,
  Testimony,
  Alibi
}

impl fmt::Display for ClueType {
   fn fmt(&self, fn: &mut fmt::Formatter<'_>)-> fmt::Result{
    write!(
      fn,
      "{},{},{}",
      self.Physical,
      self.Testimony,
      self.Alibi
    )
  } 
}


enum InterrogationResult {
  Confession,
  Denial,
  NewClue(String),
  Silent,
}

struct Clue{
  id: u32,
  description: String,
  clue_type:ClueType,
  credibility:u8
}

