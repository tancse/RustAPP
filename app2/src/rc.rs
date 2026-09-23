use std::rc::Rc;
fn main(){

    let msg = Rc::new(String::from("good evening!"));

    println!("Count:{}", Rc::strong_count(&msg));
    {
        let str1 = Rc::clone(&msg);
        println!("count:{}", Rc::strong_count(&msg));
    }

    let str2 = Rc::clone(&msg);
    println!("count:{}", Rc::strong_count(&msg));

    drop(str2);
    println!("count:{}", Rc::strong_count(&msg));
}