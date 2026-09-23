fn divide(a:i32, b:i32) -> Result<i32,String>{
    if b==0{
        Err(String::from("Cannot divide by zero"))
    }
    else{
        Ok(a/b)
    }
}

fn main(){
    // let res = divide(10,0);
    // println!("{:?}", res);

    let res = divide(10,0).unwrap();
    println!("{}",res);
}