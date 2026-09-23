fn find_student(id:i32) -> Option<String>{
    if id==1001{
        Some(String::from("harry"))
    }
    else{
        None
    }
}

fn main(){
    let result = find_student(1005);

    match result{
        Some(st_name) => println!("Student Found:{}", st_name),
        None => println!("No Student Found with Given ID")
    }
}