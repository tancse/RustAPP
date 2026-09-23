trait PrintData{
    fn print(&self);
}

struct Student{
    id:i32, name:String, age:i32
}

struct Course{
    id:i32, name:String, duration:i32
}

impl PrintData for Student{
    fn print(&self){
        println!("Student: Id: {}, Name: {}, Age: {}", self.id,self.name, self.age)
    }
}

impl PrintData for Course{
    fn print(&self){
        println!("Course: Id: {}, Name: {}, Duration: {}", self.id, self.name, self.duration)
    }
}

fn ShowData(data: &impl PrintData){
    data.print();
}

fn main() {
    let st = Student{
        id:101, name: "thann".to_string(), age:26
    };

    let cr =  Course{
        id : 91001, name: "React".to_string(), duration: 80 
    };

    // st.print();
    // cr.print();

    // ShowData(&st);
    // ShowData(&cr);

    let mut data : &dyn PrintData;
    data = &st;
    data.print();

    data = &cr;
    data.print();
}