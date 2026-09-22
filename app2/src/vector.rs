fn main() {
    let mut courses : Vec<&str> = Vec::new();          // it places the value in one memory and it is mutable it will place the replaced value of the same value at different memory

    courses.push("Java");
    courses.push("Angular");
    courses.push("Vue");
    courses.push("React");
    courses.push("NodeJS");

    println!("Before remove and pop:/n{:?}", courses);

    courses.remove(3);
    courses.pop();

    println!("After remove and pop:/n{:?}", courses);

    let mut uicourses:Vec<String>=Vec::new();       // it places the value in same memory location everytime.

    uicourses.push(String::from("NodeJs"));
    uicourses.push(String::from("JS"));
    uicourses.push(String::from("React"));
    println!("{:?}",uicourses);

    let x:&str = "heloo";
    x="abc";
    println!("{}",x)

}