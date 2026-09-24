fn main(){
    let i = Box::new(100);
    let j = &mut *i;
    *j=150;
    println!("{}",i);
    println!("{}",*i);
    println!("{}",j);
}