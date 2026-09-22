use std::io;

fn main() {
    let mut inp = String::new();

    println!("Enter Array Size:");
    io::stdin().read_line(&mut inp).unwrap();
    let size:usize = inp.trim().parse().unwrap();

    let mut courses:Vec<String>=Vec::new();

    for i in 0..size{
        inp.clear();
        println!("Enter course Name-{}:", i+1);
        io::stdin().read_line(&mut inp).unwrap();
        let coursename=inp.trim().to_string();
        courses.push(coursename);
    }

    for course in &courses{
        println!("{}",course);
    }
}