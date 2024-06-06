fn main() {
    let celsius = 20.0;
    let farenheit = 60.0;

    let c_to_f = convert_temp(celsius, 'c', 'f');
    let f_to_c = convert_temp(farenheit, 'f', 'c');
    println!("Celsius: {celsius}, farenheit: {c_to_f}");
    println!("Farenhiet: {farenheit}, celsius: {f_to_c}");
}

fn convert_temp(temperature: f64, from: char, to: char) -> f64 {
    if from == 'f' && to == 'c' {
        (temperature - 32.0) / 1.8
    } else if from == 'c' && to == 'f' {
        (temperature * 1.8) + 32.0
    } else {
        0.0
    }
}
