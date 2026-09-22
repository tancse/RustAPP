 use chrono::*;
 use chrono_tz::Tz;

fn printtime(city: &str, timezone: Tz){
    let now = Utc::now().with_timezone(&timezone);

    println!("{:<15} {}", city, now.format("%d-%m-%Y %H:%M:%S %p"))
}
fn main(){
    printtime("India", chrono_tz::Asia::Kolkata);
}