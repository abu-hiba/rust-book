fn main() {
    for i in 0..=20 {
        println!("fib{i} = {}", fib(i));
    }
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
