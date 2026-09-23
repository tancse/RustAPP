struct Employee{
    id: i32,
    name: String,
    salary: f64
}

impl Employee{
    fn display(&self)
    {
        println!("Id: {}", self.id);
        println!("Name: {}", self.name);
        println!("Salary: {}", self.salary);
    }
}

fn main() {

    let emp1 = Employee{
        id:1,
        name: "Ram".to_string(),
        salary: 65000.00
    };

    
    emp1.display();
}