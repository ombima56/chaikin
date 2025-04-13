use nalgebra::Vector2;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub position: Vector2<f64>,
    pub color: [u8; 3],
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            position: Vector2::new(x, y),
            color: [255, 255, 255], // Default white color
        }
    }

    pub fn with_color(x: f64, y: f64, color: [u8; 3]) -> Self {
        Self {
            position: Vector2::new(x, y),
            color,
        }
    }
}
