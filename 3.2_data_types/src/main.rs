fn main() {
    // addition
    let sum = 5 + 10;
    println!("sum: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product: {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("quotient: {quotient}, truncated: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {remainder}");


    // boolean
    let t = true;
    let f: bool = false;
    println!("t: {t}, f: {f}");


    // char
    let c = 'z';
    let z: char = 'ز';
    let kaba = '🕋';
    println!("c: {c}, z: {z}, kaba: {kaba}");


    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

    let tup2 = (true, 43, 2.4);
    let t2 = tup2.0;
    let forty_three = tup2.1;
    let two_point_four = tup2.2;

    println!("t2: {t2}, forty_three: {forty_three}, two_point_four: {two_point_four}");

    // array
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [6, 7, 8, 9, 10];
    let c = [3; 4];
    println!("a[0]: {}", a[0]);
    println!("b[0]: {}", b[0]);
    println!("c[0]: {}", c[0]);
}
