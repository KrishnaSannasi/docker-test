fn main() {
    println!("Hello, world from docker!");
}

#[test]
fn passing() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn failing() {
    assert_eq!(2 + 3, 4);
}
