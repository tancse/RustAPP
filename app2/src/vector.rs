fn main() {
    let mut courses : Vec<&str> = Vec::new();

    courses.push("Java");
    courses.push("Angular");
    courses.push("Vue");
    courses.push("React");
    courses.push("NodeJS");

    println!("Before remove and pop:/n{:?}", courses);

    courses.remove(3);
    courses.pop();

    println!("After remove and pop:/n{:?}", courses);
}