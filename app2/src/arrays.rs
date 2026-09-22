fn main() {
    let arr1 = [10;3];          // immutable array.
    println!("{:?}", arr1);

    let mut arr2 = [10;3];
    arr2[0] = 105;
    arr2[1] = 136;
    arr2[2] = 785;
    println!("{:?}", arr2);

    let mut courses:[&str;6] = ["Java", "Rust", "React", "Node", "Angular", "Python"];
    courses[0]="Go";
    println!("{:?}",courses);

    courses.sort();
    println!("{:?}", courses);

    courses.reverse();
    println!("{:?}", courses);
}