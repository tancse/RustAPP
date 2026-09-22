use std::io;

fn main(){

    let mut input = String::new();

    println!("Enter first value:");
    io::stdin().read_line(&mut input).unwrap();
    let a:i32=input.trim().parse().unwrap();

    input.clear();

    println!("Enter Second value:");
    io::stdin().read_line(&mut input).unwrap();
    let b:i32=input.trim().parse().unwrap();

    input.clear();

    println!("Enter third value:");
    io::stdin().read_line(&mut input).unwrap();
    let c:i32 = input.trim().parse().unwrap();

    if a>=b && a>=c{
        println!("{} is greater than {} and {}", a,b,c);
    }
    else if b>=a && b>=c{
        println!("{} is greater than {} and {}", b,a,c);
    }
    else{
        println!("{} is greater than {} and {}", c,a,b);
    }
}