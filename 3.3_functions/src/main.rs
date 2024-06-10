fn main() {
    println!("Hello, world!");

    print_labelled_measurement(5, 'h');
    function_with_scope();

    let x = five();
    println!("The value of x is: {x}");

    let z = plus_one(5);
    println!("The value of z is: {z}");
}

fn print_labelled_measurement(n: i32, unit: char) {
    println!("The measurement is: {n}{unit}");
}

fn function_with_scope() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
