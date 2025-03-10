const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}

fn main() {
    let mut temp_f = 32.0; // Starting temperature in Fahrenheit
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{:.2}°F is {:.2}°C", temp_f, temp_c);


    for i in 1..=5 {
        let next_temp_f = temp_f + i as f64;
        let next_temp_c = fahrenheit_to_celsius(next_temp_f);
        println!("{:.2}°F is {:.2}°C", next_temp_f, next_temp_c);
    }
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}