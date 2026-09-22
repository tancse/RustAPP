use std::io;

fn read_input() -> String{
    let mut input = String::new();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main(){
    println!("Enter First Number:");
    let a:i32 = read_input().trim().parse().unwrap();

    println!("Enter Second Number:");
    let b:i32 = read_input().trim().parse().unwrap();

    println!("Enter Third Number:");
    let c:i32 = read_input().trim().parse().unwrap();

    if a>=b && a>=c {
        println!("{} is greater than {}  and {}", a,b,c);
    }
    else if b>c {
        println!("{} is greater than {}  and {}", b,a,c);
    }
    else{
        println!("{} is greater than {}  and {}", c,a,b);
    }

}