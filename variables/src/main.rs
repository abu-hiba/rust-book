fn main() {
    let mut a = 5;
    println!("The value of a is: {a}");
    a = 6;
    println!("The value of a is: {a}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x is {x}");
}
