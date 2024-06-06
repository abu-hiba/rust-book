fn main() {
    let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    let y: Option<i8> = None;

    let sum = match y {
        Some(val) => x + val,
        None => x + 0,
    };

    println!("x + y is: {sum}");
}
