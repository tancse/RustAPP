struct Address{
    city:String, state:String
}

struct Employee{
    id:i32, name: String, address: Address
}

impl Employee{
    fn new(id:i32, name:String, address:Address) -> Employee {
        Employee{
            id, name, address
        }
    }
    fn display(&self){
        println!("Id: {}", self.id);
        println!("Name: {}", self.name);
        println!("City: {}", self.address.city);
        println!("State: {}", self.address.state);
    }
}

fn main(){
    // let emp1 = Employee{
    //     id:101,
    //     name: "Ram".to_string(),
    //     address: Address{
    //         city: "Chennai".to_string(),
    //         state: "Tamilnadu".to_string()
    //     }
    // };

    let emp1 = Employee::new(101, "Ram".to_string(), Address{city:"Chennai".to_string(), state:"Tamilnadu".to_string()});
    emp1.display();
}
