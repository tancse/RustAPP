use std::io;

fn main(){
    let mut inp = String::new();

    println!("Enter First value:");
    io::stdin().read_line(&mut inp).unwrap();
    let a:i32 = inp.trim().parse().unwrap();
    
    inp.clear();

    println!("Enter First value:");
    io::stdin().read_line(&mut inp).unwrap();
    let b:i32 = inp.trim().parse().unwrap();

    loop{
        inp.clear();

        println!("Enter any choice:\n1.Add\n2.Sub\n3.Multiply\n4.division");
        io::stdin().read_line(&mut inp).unwrap();
        let i:i32 = inp.trim().parse().unwrap();

        match i{
            1 => println!("{}",a+b),
            2 => println!("{}", a-b),
            3 => println!("{}", a*b),
            4 => println!("{}", a/b),
            0 => {println!("Exiting Program!"); break;}
            _ => println!("Invalid Option!") 
        }
    }

}