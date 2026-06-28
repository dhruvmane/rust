fn main() {
    let s1 = String::from("Hello");
    let length = calculate_length(&s1);
    let s2 = &s1;

    println!("The length of '{}' is {}.", s2, length);
}

fn calculate_length(s: &String) -> usize {
    
    // Return the length.
    s.len()
}