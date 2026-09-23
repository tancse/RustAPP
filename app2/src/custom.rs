struct inSufficientFunds;

fn withdraw(balance:f64, amount:f64) -> Result<f64, inSufficientFunds>{
    if amount>balance{
        Err(inSufficientFunds)
    }
    else{
        Ok(balance-amount)
    }
}

fn main(){
    let res = withdraw(6000.0, 7000.0);

    match res{
        Ok(balance) => println!("Transaction is successfull. Available Balance:{}", balance),
        Err(_) => println!("Error: Insufficient Funds in Account")
    }
}