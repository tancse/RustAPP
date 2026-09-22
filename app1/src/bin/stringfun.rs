use std::io;
fn main(){
    let mut first = String::new();
    let mut last = String::new();

    println!("Enter your first name");
    io::stdin().read_line(&mut first).unwrap();

    println!("Enter your second name");
    io::stdin().read_line(&mut last).unwrap();

    greet(first.trim().to_string(), last.trim().to_string())
}

fn greet(fname:String, lname:String){
    println!("Welcome, {} {}", fname, lname);
}