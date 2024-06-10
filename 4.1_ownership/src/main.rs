fn main() {
    let s1 = String::from("hello");

    // let s2 = s1;
    // println!("{s1}"); borrow of moved value: `s1`
    
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    takes_ownership(s2);

    let x = 5;

    makes_copy(x);

    // println!("s2 = {s2}, x = {x}");  borrow of moved value: `s2`
    println!("x = {x}");


    let s3 = gives_ownership();
    let s4 = String::from("hello");
    let s5 = takes_and_gives_back(s4);

    println!("{s3}, {s5}");
}
// s1 dropped
// s2 was moved so not dropped
// x is not dropped because it is on the stack
// s3 is dropped
// s4 was moved
// s5 is dropped

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
