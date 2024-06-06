fn main() {
    let number = 3;
    
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number is not zero");
    }

    let condition = true;

    let number2 = if condition { 5 } else { 4 };

    println!("number2 is: {number2}");
}
