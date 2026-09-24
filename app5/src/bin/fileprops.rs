use std::fs;
use std::path::Path;

fn main(){
    let fpath = Path::new(r"demo.txt");

    println!("File Path:{}", fpath.display());

    if let Some(name) = fpath.file_name(){
        println!("File Name:{}", name.to_string_lossy());
    }

    if let Some(stem) = fpath.file_stem(){
        println!("only File Name:{}", stem.to_string_lossy());
    }

    if let Some(extension) = fpath.extension(){
        println!("extension only File Name:{}", extension.to_string_lossy());
    }

    if let Some(parent) = fpath.parent(){
        println!("parent only File Name:{}", parent.to_string_lossy());
    }

    println!("Is it a file:{}", fpath.is_file());
    println!("Is it a folder:{}", fpath.is_dir());

    if let Ok(metadata)=fs::metadata(fpath){
        println!("file size:{}", metadata.len());
    }
}