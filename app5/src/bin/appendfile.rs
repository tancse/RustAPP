use std::fs::*;
use std::path::Path;
use std::io::*;

fn main(){
    let fpath = "demo.txt";
    if Path::new(fpath).exists(){
        let mut file = OpenOptions::new()
                        .append(true)
                        .open(fpath)
                        .unwrap();
        writeln!(file, "\nThis is line1").unwrap();
        writeln!(file, "This is line2").unwrap();
        writeln!(file, "This is line3").unwrap();
    }
    else{
        println!("Incorrect file Path/ File not found!");
    }
}