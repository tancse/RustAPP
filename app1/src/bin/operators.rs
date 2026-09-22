use std::{io, string};

fn sum(a: i32, b: i32) -> i32{
    a+b
}

fn sub(a: f32, b: f32) -> f32{
    a-b
}

fn main() {
    let mut val1 = String::new();
    let mut val2 = String::new();

    println!("Enter First Value:");
    io::stdin().read_line(&mut val1).unwrap();
    println!("Enter second Value:");
    io::stdin().read_line(&mut val2).unwrap();

    let x:i32=val1.trim().parse().unwrap();
    let y:i32=val2.trim().parse().unwrap();

    let res = sum(x,y);
    println!("sum = {}", res);
    
    let mut valf1 = String::new();
    let mut valf2 = String::new();

    println!("Enter First float Value:");
    io::stdin().read_line(&mut valf1).unwrap();
    println!("Enter second float Value:");
    io::stdin().read_line(&mut valf2).unwrap();

    let xf:f32=valf1.trim().parse().unwrap();
    let yf:f32=valf2.trim().parse().unwrap();

    let min = sub(xf, yf);
    println!("sub = {:.2}", min);
}