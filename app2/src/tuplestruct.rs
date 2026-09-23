struct Employee(i32,String,f64);

fn main(){
    let emp=Employee(1,String::from("Krishna"),55000.0);

    println!("Id:{}",emp.0);
    println!("Name:{}",emp.1);
    println!("Salary:{}",emp.2);

}