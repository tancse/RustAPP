use std::fmt::Display;

struct course<T>{
    id:T,
    name:String
}

impl<T: Display> course<T>{
    fn print(&self){
        println!("Id:{}", self.id);
        println!("Name:{}", self.name);
    }
}

fn main() {
    let course1 = course::<i32>{
        id:101,
        name: "React".to_string()
    };

    let course2 = course::<&str>{
        id:"UI2589",
        name: "JAVA".to_string()
    };

    // println!("Courses1: Id:{}  Title:{}", course1.id, course1.name);
    // println!("Courses1: Id:{}  Title:{}", course2.id, course2.name);

    course1.print();
    course2.print();
}