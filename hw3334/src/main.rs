const FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING_POINT
}

fn main() {
    // Declares mutable variable with temp in F
    let mut temp_f: f64 = 32.0;

    // Converts it to C using function and prints result
    let cel_temp = fahrenheit_to_celsius(temp_f);
    println!("{}°F is {:.2}°C", temp_f, cel_temp);

    for _ in 1..=5 {
        temp_f += 1.0;
        let cel_temp = fahrenheit_to_celsius(temp_f);
        println!("{}°F is {:.2}°C", temp_f, cel_temp);
    }

}