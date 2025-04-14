#[cfg(test)]
mod tests {
    use super::super::point::Point;
    use nalgebra::Vector2;


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
