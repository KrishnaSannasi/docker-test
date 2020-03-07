fn main() {
    println!("Hello, world from docker!");
}

#[test]
fn passing() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn passing_2() {
    assert_ne!(2 + 3, 4);
}
