use std::collections::HashMap;

fn main(){
    let mut courses = HashMap::new();

    courses.insert(10201, "Java");
    courses.insert(10101, "SQL");
    courses.insert(10203, "Python");
    courses.insert(10932, "Oracle");

    println!("{:?}", courses);
    println!("Course Name:{:?}", courses.get(&10203));

    courses.remove(&10101);
    println!("{:?}",courses);

    courses.insert(10101, "Mongo");     // key is constant, value can be overridded
    println!("{:?}", courses);

    courses.insert(20345, "JAVA");      // multiple keys or duplication of values are allowed
    println!("{:?}", courses);
}