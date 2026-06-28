fn main() {
    let number = 3;

    if number < 5 {
        println!("The condition was true.");
    } else {
        println!("The condition was false.");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {number}");


    for n in (1..4) {
        println!("the number is: {n}");
    }

}
