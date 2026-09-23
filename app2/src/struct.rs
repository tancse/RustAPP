struct Employee{
    id: i32,
    name: String,
    salary: f64
}

fn main() {

    let emp1 = Employee{
        id:1,
        name: "Ram".to_string(),
        salary: 65000.00
    };

    println!("Id: {}", emp1.id);
    println!("Name: {}", emp1.name);
    println!("Salary: {}", emp1.salary);
}