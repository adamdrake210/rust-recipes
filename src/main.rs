extern crate d1_filework;
use d1_filework::*;
use failure::Error;

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let trans = get_transactions_b("test_data/transactions.json")?;
    for t in trans {
        println!("{:?}", t);
    }

    // First transaction
    let t = get_first_transaction_for("test_data/transactions.json", "Matt");
    match t {
      Ok(v) => println!("Found transaction for: {:?}", v),
      Err(e) => println!("Error: {}, Backtrace = : {:?}", e, e.backtrace()),
    }
    
    // First transaction that goes to Adam
    let to = get_first_transaction_to("test_data/transactions.json", "Ada");

    match to {
      Ok(v) => println!("Found transaction to: {:?}", v),
      Err(e) => println!("Error: {}, Backtrace = : {:?}", e, e.backtrace()),
    }
    
    
    Ok(())

}

