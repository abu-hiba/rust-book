fn main() {
    // let config_max = Some(3u8);
    let config_max: Option<u8> = None;

    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
    
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("No maximum found");
    }
}
