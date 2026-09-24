use std::fs::*;

fn main(){
    create_dir_all(r"C:/RustIO").unwrap();

    File::create(r"C:/RustIO/sample.txt").unwrap();

    println!("Folder and File Created Successfully");
}