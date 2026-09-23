use std::io;

fn main() {
    let mut products:Vec<(i32,String,f64,String)>=Vec::new();
    let mut inp = String::new();

    println!("Enter No. of Products:");
    io::stdin().read_line(&mut inp).unwrap();
    let size:i32 = inp.trim().parse().unwrap();

    for i in 1..=size{
        inp.clear();
        println!("Enter ID:");
        io::stdin().read_line(&mut inp).unwrap();
        let id:i32=inp.trim().parse().unwrap();

        inp.clear();
        println!("Enter Name:");
        io::stdin().read_line(&mut inp).unwrap();
        let name=inp.trim().to_string();

        inp.clear();
        println!("Enter Price:");
        io::stdin().read_line(&mut inp).unwrap();
        let price:f64=inp.trim().parse().unwrap();

        inp.clear();
        println!("Enter Category:");
        io::stdin().read_line(&mut inp).unwrap();
        let category=inp.trim().to_string();

        products.push((id,name,price,category));
        
    }

    for product in &products{
        println!("Id:{}\tName:{}\tPrice:{}\tCategory:{}",product.0, product.1,product.2,product.3);
    }
}