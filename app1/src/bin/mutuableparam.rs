fn main(){
    let mut x = 100;
    
    println!("Before fn call:{}",x);

    show_number(&mut x);   // fuction updating the outer variable that is the parameter value after the fun using &mut, which gives the write access.

    println!("After fn call:{}",x);


}

fn show_number(a: &mut i32){
    *a = *a + 1;
}