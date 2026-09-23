use std::collections::LinkedList;

fn main() {
    let mut llist = LinkedList::new();

    llist.push_back(25);
    llist.push_back(50);
    llist.push_back(93);
    llist.push_back(25);
    llist.push_back(74);
    llist.push_back(86);

    println!("{:?}", llist);

    llist.pop_front();
    println!("First element deleted: {:?}", llist);

    llist.pop_back();
    println!("Last element deleted: {:?}", llist);
}