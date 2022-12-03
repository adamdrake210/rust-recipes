extern crate d1_filework;
use d1_filework::*;

fn main() -> Result<(), TransactionError> {
    println!("Hello, world!");
    let trans = get_transactions_b("test_data/transactions.json")?;
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

