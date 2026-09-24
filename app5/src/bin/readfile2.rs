use std::fs::File;
use std::io::*;

fn main() {
    let file = File::open("demo.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines(){
        println!("{}", line.unwrap());
    }
}