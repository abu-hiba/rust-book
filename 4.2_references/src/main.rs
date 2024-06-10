fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}");

    change(&mut s1);

    let r1 = &mut s1;
    // let r2 = &mut s1; // cannot do this because the r1 reference has not be used yet
    println!("{r1}");

    let r2 = &s1;
    let r3 = &s1;
    // let r4 = &mut s1; // cannot create mutable reference when immutable references have not been
    // used for the final time

    println!("r2: {r2}, r3: {r3}"); // r2 and r3 are not used after here so a mutable reference can
    // be created

    let r4 = &mut s1; // no problem
    println!("r4: {r4}");

    // dangling reference
    let s_from_dangle = dangle();
    println!("{s_from_dangle}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
// s goes out of scope but because it does not have ownership
// of what it refers to, it is not dropped

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello");

    // &s // return a reference to s
    s
}
// s goes out of scope and is dropped, so the reference is pointing to an invalid String
