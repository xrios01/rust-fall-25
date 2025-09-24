// Assignment 1: Temperature Converter
const FREEZING_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9.0 / 5.0) + 32.0
}

fn main() {
    // Start at the freezing point
    let mut temp_f: f64 = FREEZING_F;

    let temps:[f64; 5] = [33.0, 34.0, 35.0, 36.0, 37.0];

    // Convert starting temp to Celsius
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{temp_f:.0}°F = {temp_c:.2}°C");

    // Convert and print the next 5 integer Fahrenheit temperatures
    for temp in temps {
        let c = fahrenheit_to_celsius(temp);
        println!("{temp:.0}°F = {c:.2}°C");
    }

    
    // let back_to_f = celsius_to_fahrenheit(temp_c);
    // println!("Convert back: {temp_c:.2}°C = {back_to_f:.2}°F");
}
