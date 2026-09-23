struct Employee{
    id:i32, name:String, department:String, status:EmployeeStatus
}

enum EmployeeStatus{
    Active,
    Onleave,
    Resigned
}

fn print_employee( emp: &Employee){
    println!("ID: {}", emp.id);
    println!("Name: {}", emp.name);
    println!("department: {}", emp.department);
    match &emp.status{
        EmployeeStatus::Active => {
            println!("Status: Active");
        },
        EmployeeStatus::Onleave => {
            println!("Status: Onleave");
        },
        EmployeeStatus::Resigned => {
            println!("Status: Resigned");
        }
    }
}

fn main() {
    let emp1 = Employee{
        id : 1001,
        name: "Thananya".to_string(),
        department: "Finance".to_string(),
        status : EmployeeStatus::Active
    };
    print_employee(&emp1);
}