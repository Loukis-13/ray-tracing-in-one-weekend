pub use crate::vec3::Vec3 as Color;

impl Color {
    fn clamp(x: f64, min: f64, max: f64) -> f64 {
        if x < min { return min; }
        if x > max { return max; }
        x
    }
    
    pub fn color_to_u8(&self, samples_per_pixel: i32) -> [u8; 3] {
        let Color(mut x, mut y,mut z) = self;
        let scale = 1.0 / samples_per_pixel as f64;
    
        x = (x * scale).sqrt();
        y = (y * scale).sqrt();
        z = (z * scale).sqrt();
    
        return [
            (255.999 * Self::clamp(x, 0.0, 0.999)) as u8,
            (255.999 * Self::clamp(y, 0.0, 0.999)) as u8,
            (255.999 * Self::clamp(z, 0.0, 0.999)) as u8,
        ];
    }
}
