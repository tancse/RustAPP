use std::collections::*;
fn main() {
    let mut queue:VecDeque<i32>=VecDeque::new();

    queue.push_back(85);
    queue.push_back(63);
    queue.push_back(27);
    queue.push_back(85);
    queue.push_back(91);
    queue.push_back(73);

    println!("{:?}", queue);

    queue.pop_front();
    println!("{:?}", queue);
}