fn main(){
    let x = 100;
    show_number(&x); // & makes the program to only read the value
    println!("The Value After Calling function is: {}", x);
}

fn show_number(a:&i32){ // analogus to pass by reference, in RUST it is borrowing.
    println!("Value Received is:{}", a);
}