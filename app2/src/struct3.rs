struct Employee{
    id: i32,
    name: String,
    salary: f64
}

impl Employee{
    fn new(id:i32, name:String, salary:f64) -> Employee{
        Employee{
            id,name,salary
        }
    }
    fn display(&self)
    {
        println!("Id: {}", self.id);
        println!("Name: {}", self.name);
        println!("Salary: {}", self.salary);
    }
}

fn main() {

    let emp1 = Employee::new(1,"Ram".to_string(),65000.00);

    emp1.display();
}