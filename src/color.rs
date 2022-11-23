use crate::vec3::Color;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    x
}

pub fn write_color(color: Color, samples_per_pixel: i32) {
    let Color(mut x, mut y,mut z) = color;
    let scale = 1.0 / samples_per_pixel as f64;

    x = (x * scale).sqrt();
    y = (y * scale).sqrt();
    z = (z * scale).sqrt();

    println!(
        "{} {} {}",
        (255.999 * clamp(x, 0.0, 0.999)) as u8,
        (255.999 * clamp(y, 0.0, 0.999)) as u8,
        (255.999 * clamp(z, 0.0, 0.999)) as u8
    )
}
