use std::panic;

fn main() {
    let result = panic::catch_unwind(||{
        println!("Some code execution");

        panic!("oops something went wrong");
    });

    match result{
        Ok(_)=>println!("execution completed successfully"),
        Err(_)=>println!("Panci was handled"),
    }

    println!("job completed");
}