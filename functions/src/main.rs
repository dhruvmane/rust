fn main() {
    println!("Hello, world!");
    another_function();
    five(5);
    print_labelled_measurement(5, 'm');
}

fn another_function() {
    println!("This is another function.");
}

fn five(x: i32) {
    println!("the value of x is {x}");
}

fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("the measurement is: {value}{unit_label}");
}