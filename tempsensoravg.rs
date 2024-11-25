fn main() {
    let temperatures = [10.2, 11.3, 10.7, 12.0, 12.0, 11.2, 11.9];
    
    let mean_celsius = calculate_mean(&temperatures);
    println!("Mean temperature in Celsius: {:.1}", mean_celsius);
    
    let mean_fahrenheit = celsius_to_fahrenheit(mean_celsius);
    println!("Mean temperature in Fahrenheit: {:.0}", mean_fahrenheit);
    
    let (min_temp, max_temp) = find_min_max(&temperatures);
    println!("Minimum temperature recorded: {:.1} °C", min_temp);
    println!("Maximum temperature recorded: {:.1} °C", max_temp);
}

/* Helper function definitions after main - why here idiomatically (as not a strict rule):
   1. main() is (eventual) entry point, so traditionally listed as first function
   2. putting main first allows us to see high-level logic flow - then we can scroll
      to look at the helpers' actual detail.
*/

fn calculate_mean(temperatures: &[f64]) -> f64 {
    let sum: f64 = temperatures.iter().sum();
    sum / temperatures.len() as f64
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0/5.0 + 32.0
}

fn find_min_max(temperatures: &[f64]) -> (f64, f64) {
    let min_temp = temperatures.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_temp = temperatures.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    (min_temp, max_temp)
}
