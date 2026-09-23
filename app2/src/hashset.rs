use std::collections::*;

fn main() {
    let mut scores = HashSet::new();

    scores.insert(65);
    scores.insert(60);
    scores.insert(75);
    scores.insert(81);
    scores.insert(65);
    scores.insert(92);

    println!("Before remove: {:?}", scores);
    println!("{:?}", scores.len());

    scores.remove(&92);
    println!("After remove: {:?}", scores);
    println!("{:?}", scores.len());
}