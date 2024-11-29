use std::cmp::PartialOrd;

/// A trait to bind a value within a specified minimum and maximum range.
trait BoundedValue {
    /// Clamps the given value between the provided minimum and maximum bounds.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum bound.
    /// * `max` - The maximum bound.
    ///
    /// # Returns
    ///
    /// The clamped value.
    fn clamp_value(self, min: Self, max: Self) -> Self;
}

/// Implement BoundedValue for any type that is:
/// - PartialOrd: Allows comparison between values.
/// - Copy: Enables copying the value instead of moving it.
impl<T> BoundedValue for T
where
    T: PartialOrd + Copy,
{
    fn clamp_value(self, min: T, max: T) -> T {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

fn main() {
    // Using with integer type (i32)
    let int_val: i32 = 15;
    let int_min: i32 = 10;
    let int_max: i32 = 20;
    let clamped_int = int_val.clamp_value(int_min, int_max);
    println!("Clamped integer: {}", clamped_int); // Output: 15

    let int_val_exceed: i32 = 25;
    let clamped_int_exceed = int_val_exceed.clamp_value(int_min, int_max);
    println!("Clamped integer (exceeds max): {}", clamped_int_exceed); // Output: 20

    // Using with floating-point type (f64)
    let float_val: f64 = 3.14;
    let float_min: f64 = 0.0;
    let float_max: f64 = 10.0;
    let clamped_float = float_val.clamp_value(float_min, float_max);
    println!("Clamped float: {}", clamped_float); // Output: 3.14

    let float_val_below: f64 = -5.0;
    let clamped_float_below = float_val_below.clamp_value(float_min, float_max);
    println!("Clamped float (below min): {}", clamped_float_below); // Output: 0.0
}
