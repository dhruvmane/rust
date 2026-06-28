fn main() {
    // let mut s: String = String::from("hello");
    // s.push_str(", world!");
    // println!("the String is: {s}");

    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("{}, world!", s1);
    println!("{}, world!", s2);

    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}