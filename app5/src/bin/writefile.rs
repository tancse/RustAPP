use std::fs;
use std::path::Path;

fn main() {
    let fpath = "demo.txt";
    if Path::new(fpath).exists(){
        fs::write(fpath,"This is Line-1").unwrap();
        fs::write(fpath,"This is Line-2").unwrap();
    }
    else{
        println!("Incorrect file Path/ File not found!");
    }
}
