mod error;
pub use error::TransactionError;
use serde_derive::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
  from: String,
  to: String,
  amount: u64,
}


pub fn get_first_transaction_for(fname: &str, uname: &str) -> Result<Transaction, failure::Error> {
  let trans = get_transactions_b(fname)?;

  for t in trans {
    if t.from == uname {
      return Ok(t);
    }
  }
  Err(TransactionError::Mess("Could not find transaction").into())
}

pub fn get_first_transaction_to(fname: &str, uname: &str) -> Result<Transaction, failure::Error> {
  let trans = get_transactions_b(fname)?;

  for t in trans {
    if t.to == uname {
      return Ok(t);
    }
  }
  Err(TransactionError::Mess("Could not find transaction").into())
}

// pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, String> {
//   // Err("No trans".to_string())
//   let s = match std::fs::read_to_string(fname) {
//     Ok(v) => v,
//     Err(e) => return Err(e.to_string()),
//   };
//   let t:Vec<Transaction> = match serde_json::from_str(&s) {
//     Ok(v) => v,
//     Err(e) => return Err(e.to_string()),
//   };

//   Ok(t)
// }

pub fn get_transactions_b(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
  std::fs::read_to_string(fname)
    .map_err(|e| e.into())
    .and_then(|s| serde_json::from_str(&s).map_err(|e| e.into()))
}