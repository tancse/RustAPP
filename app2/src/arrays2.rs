use std::io;
fn main() {
    let mut arr = [0;100];

    let mut inp = String::new();

    inp.clear();

    println!("Enter array size:");
    io::stdin().read_line(&mut inp).unwrap();
    let size:usize = inp.trim().parse().unwrap();

    for i in 0..size{
        println!("Enter value for Index-{}", i);

        let mut arrinp = String::new();
        io::stdin().read_line(&mut arrinp).unwrap();
        arr[i]=arrinp.trim().parse().unwrap();
    }

    println!("{:?}", arr);

    for j in 0..size{
        print!("{} \t", arr[j]);
    }
    println!("");

    println!("{:?}", &arr[0..size]);
    
}