use serde_derive::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
  from: String,
  to: String,
  amount: u64,
}
#[derive(Debug)]
pub enum TransactionError {
  LoadError(std::io::Error),
  ParseError(serde_json::Error),
  Mess(&'static str),
}

impl From<std::io::Error> for TransactionError {
  fn from(err: std::io::Error) -> Self {
    TransactionError::LoadError(err)
  }
}

impl From<serde_json::Error> for TransactionError {
  fn from(err: serde_json::Error) -> Self {
    TransactionError::ParseError(err)
  }
}

impl From<&'static str> for TransactionError {
  fn from(err: &'static str) -> Self {
    TransactionError::Mess(err)
  }
}

fn main() -> Result<(), TransactionError> {
    println!("Hello, world!");
    let trans = get_transactions_b("test_data/transactions.json").expect("Could not load transactions");
    for t in trans {
        println!("{:?}", t);
    }

    // First transaction
    let t = get_first_transaction_for("test_data/transactions.json", "Matt").ok_or("Could not get first transaction")?;

    println!("First transaction for Adam: {:?}", t);
    
    // First transaction that goes to Adam
    let to = get_first_transaction_to("test_data/transactions.json", "Adam").ok_or("Could not get fist transaction to")?;
    println!("First transaction that goes to: {:?}", to);
    
    Ok(())

}

fn get_first_transaction_for(fname: &str, uname: &str) -> Option<Transaction> {
  let trans = get_transactions_b(fname).ok()?;

  for t in trans {
    if t.from == uname {
      return Some(t);
    }
  }
  None
}

fn get_first_transaction_to(fname: &str, uname: &str) -> Option<Transaction> {
  let trans = get_transactions_b(fname).ok()?;

  for t in trans {
    if t.to == uname {
      return Some(t);
    }
  }
  None
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

fn get_transactions_b(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
  std::fs::read_to_string(fname)
    .map_err(|e| e.into())
    .and_then(|s| serde_json::from_str(&s).map_err(|e| e.into()))
}