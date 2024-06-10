fn main() {
    let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    let y: Option<i8> = None;

    let sum = match y {
        Some(val) => x + val,
        None => x + 0,
    };

    println!("x + y is: {sum}");


    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}
