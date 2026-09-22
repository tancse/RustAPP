use chrono::{Local, Datelike, Timelike, Duration, Months};

fn main() {

    let dt=Local::now();
    println!("Current Date and Time {}", dt);
    
    println!("Current Date and Time in custo format {}", dt.format("%d-%m-%Y %H:%M:%S"));

    println!("Current Date and Time in custo format {}", dt.format("%d-%m-%Y %I:%M:%S %p"));

    println!("Year:{}", dt.year());
    println!("Month:{}", dt.month());
    println!("Day:{}", dt.day());
    println!("weekDay:{}", dt.weekday());

    println!("Hour:{}", dt.hour());
    println!("Minutes:{}", dt.minute());
    println!("Seconds:{}", dt.second());

    let future=dt+Duration::days(10345);
    println!("future date: {}", future.format("%d-%m-%Y"));

    let future1=dt+Months::new(10345);
    println!("future month: {}", future1.format("%d-%m-%Y"));

    // let future2=dt+Years::new(10345);
    // println!("future year: {}", future2.format("%d-%m-%Y"));

    let future3=dt+Duration::weeks(10345);
    println!("future date: {}", future3.format("%d-%m-%Y"));

    let future4=dt+Duration::hours(10345);
    println!("future date: {}", future4.format("%d-%m-%Y %H:%M:%S"));

    let future5=dt+Duration::minutes(10345);
    println!("future date: {}", future5.format("%d-%m-%Y %H:%M:%S"));

    


    
}