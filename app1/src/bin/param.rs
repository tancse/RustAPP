fn main(){
    let x = 100;
    show_number(x);
    println!("The Value After Calling function is: {}", x);
}

fn show_number(a:i32){
    println!("Value Received is:{}", a);
}