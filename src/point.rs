use nalgebra::Vector2;

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub position: Vector2<f64>,
    pub color: [u8; 3],
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            position: Vector2::new(x, y),
            color: [255, 255, 255], 
        }
    }

    pub fn with_color(x: f64, y: f64, color: [u8; 3]) -> Self {
        Self {
            position: Vector2::new(x, y),
            color,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // bring in Point and Vector2

    #[test]
    fn test_point_new() {
        let point = Point::new(1.0, 2.0);
        assert_eq!(point.position, Vector2::new(1.0, 2.0));
        assert_eq!(point.color, [255, 255, 255]);
    }

    #[test]
    fn test_point_with_color() {
        let color = [100, 150, 200];
        let point = Point::with_color(3.0, 4.0, color);
        assert_eq!(point.position, Vector2::new(3.0, 4.0));
        assert_eq!(point.color, color);
    }
}
