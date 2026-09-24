use std::fs;

fn main(){
    let filetext = fs::read("demo.txt").unwrap();
    let text = String::from_utf8(filetext).unwrap();
    println!("{}",text)
}