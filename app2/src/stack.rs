fn main() {
    let mut stack:Vec<i32>=Vec::new();

    stack.push(25);
    stack.push(89);
    stack.push(10);
    stack.push(47);
    stack.push(25);
    stack.push(63);

    println!("{:?}", stack);

    stack.pop();
    println!("{:?}", stack);

    stack.pop();
    println!("{:?}", stack);

}