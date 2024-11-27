fn main() {
    let a = 2.0; // Example side length of the dodecahedron (you can change this value)

    // Calculate volume
    let volume = calculate_volume(a);
    println!("Volume of the dodecahedron: {:.2}", volume);

    // Calculate surface area
    let surface_area = calculate_surface_area(a);
    println!("Surface area of the dodecahedron: {:.2}", surface_area);
}

// Function to calculate the volume of a dodecahedron
fn calculate_volume(a: f64) -> f64 {
    let constant = (15.0 + 7.0 * f64::sqrt(5.0)) / 4.0;
    constant * a.powi(3)  // a^3
}

// Function to calculate the surface area of a dodecahedron
fn calculate_surface_area(a: f64) -> f64 {
    let constant = 3.0 * f64::sqrt(25.0 + 10.0 * f64::sqrt(5.0));
    constant * a.powi(2)  // a^2
}
