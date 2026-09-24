fn main(){
    let str1=String::from("india");
    println!("Value of str1:{}", str1);

    let str2=str1;
    println!("Value of str2:{}", str2);
    //println!("Value of str1:{}", str1);

    let i1 =10;
    println!("Value of i1:{}",i1);
    let i2 = i1;
    println!("value of i2:{}", i2);
    println!("Value of i1:{}",i1);

    let str3 = String::from("India");
    println!("Value of str3:{}",str3);

    printvalues(&str3);

    println!("Value of str3:{}",str3);

}

fn printvalues(country:&String){
    println!("Value of country:{}", country);
}