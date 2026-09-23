enum coursestate{
    NotStarted,
    InProgress,
    Cancelled,
    Completed
}

fn main() {
    let status = coursestate :: NotStarted;
    
    match status{
        coursestate::NotStarted =>{
            println!("Course has not started");
        }
        coursestate::InProgress =>{
            println!("Course has been InProgress");
        }
        coursestate::Cancelled =>{
            println!("Course has been Cancelled");
        }
        coursestate::Completed =>{
            println!("Course has been Completed");
        }
    }
}