fn main() {
    println!("Hello, world!");
    let mut s = String::from("Hello");
    let t = "World!";
    s.push_str(t);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}
