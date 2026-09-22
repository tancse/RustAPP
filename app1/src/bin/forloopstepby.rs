use std::io;

fn read_input() -> String{
    let mut input = String::new();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main(){
    println!("Enter First Number:");
    let n:i32 = read_input().trim().parse().unwrap();

    for i in (1..=n).rev().step_by(2){
        println!("{}",i);
    }

}