
fn check_age(age:i32) -> Result<i32, String>{
    if age<18{
        Err(String::from("Not Eligible"))
    }
    else{
        Ok(age)
    }
}

fn without() -> Result<i32,String>{
    match check_age(10){
        Ok(age) => Ok(age),
        Err(error) => Err(error),
    }
}

fn withop() -> Result<i32, String>{
    let age = check_age(10)?;
        Ok(age)
}

fn main() {
    println!("without : {:?}", without());
    println!("Withop: {:?}", withop());
}









