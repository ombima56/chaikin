use crate::chaikin::*;

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::point::Point;
    use nalgebra::Vector2;

    fn point(x: f64, y: f64) -> Point {
        Point::new(x, y)
    }

    #[test]
    fn test_new_initializes_correctly() {
        let input = vec![point(0.0, 0.0), point(10.0, 10.0)];
        let chaikin = Chaikin::new(input.clone());
        assert_eq!(chaikin.original_points, input);
        assert_eq!(chaikin.current_points, input);
        assert!(chaikin.next_points.is_empty());
        assert_eq!(chaikin.animation_progress, 0.0);
        assert_eq!(chaikin.current_step, 0);
    }

    #[test]
    fn test_apply_chaikin_generates_correct_number_of_points() {
        let input = vec![point(0.0, 0.0), point(10.0, 10.0)];
        let chaikin = Chaikin::new(input.clone());

        let output = chaikin.apply_chaikin(&input);
        // Should have original endpoints + 2 new points per segment
        assert_eq!(output.len(), 4);
    }

    #[test]
    fn test_interpolate_same_length() {
        let mut chaikin = Chaikin::new(vec![]);
        chaikin.current_points = vec![point(0.0, 0.0), point(2.0, 2.0)];
        chaikin.next_points = vec![point(2.0, 2.0), point(4.0, 4.0)];

        let result = chaikin.interpolate(0.5);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].position, Vector2::new(1.0, 1.0));
        assert_eq!(result[1].position, Vector2::new(3.0, 3.0));
    }

    #[test]
    fn test_interpolate_different_point_counts_more_current() {
        let mut chaikin = Chaikin::new(vec![]);
        chaikin.current_points = vec![
            point(0.0, 0.0),
            point(2.0, 2.0),
            point(4.0, 4.0),
            point(6.0, 6.0),
        ];
        chaikin.next_points = vec![
            point(0.0, 0.0),
            point(6.0, 6.0),
        ];

        let result = chaikin.interpolate_different_point_counts(0.5);
        assert_eq!(result.first().unwrap().position, Vector2::new(0.0, 0.0));
        assert_eq!(result.last().unwrap().position, Vector2::new(6.0, 6.0));
    }

    #[test]
    fn test_interpolate_different_point_counts_more_next() {
        let mut chaikin = Chaikin::new(vec![]);
        chaikin.current_points = vec![
            point(0.0, 0.0),
            point(6.0, 6.0),
        ];
        chaikin.next_points = vec![
            point(0.0, 0.0),
            point(2.0, 2.0),
            point(4.0, 4.0),
            point(6.0, 6.0),
        ];

        let result = chaikin.interpolate_different_point_counts(0.5);
        assert_eq!(result.first().unwrap().position, Vector2::new(0.0, 0.0));
        assert_eq!(result.last().unwrap().position, Vector2::new(6.0, 6.0));
    }

    #[test]
    fn test_create_visualization_adds_colors() {
        let input = vec![point(1.0, 1.0), point(2.0, 2.0)];
        let chaikin = Chaikin::new(vec![point(0.0, 0.0)]);
        let result = chaikin.create_visualization(input.clone());

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].color, [255, 0, 0]); // original
        assert_eq!(result[1].color, [0, 255, 0]); // interpolated
        assert_eq!(result[2].color, [0, 255, 0]); // interpolated
    }

    #[test]
    fn test_set_points_resets_state() {
        let mut chaikin = Chaikin::new(vec![point(1.0, 1.0)]);
        chaikin.animation_progress = 0.5;
        chaikin.current_step = 3;

        chaikin.set_points(vec![point(2.0, 2.0)]);

        assert_eq!(chaikin.original_points.len(), 1);
        assert_eq!(chaikin.original_points[0].position, Vector2::new(2.0, 2.0));
        assert_eq!(chaikin.animation_progress, 0.0);
        assert_eq!(chaikin.current_step, 0);
    }

    #[test]
    fn test_set_points_does_nothing_if_same() {
        let original = vec![point(3.0, 3.0)];
        let mut chaikin = Chaikin::new(original.clone());

        chaikin.animation_progress = 0.7;
        chaikin.set_points(original.clone());

        // Should not reset
        assert_eq!(chaikin.animation_progress, 0.7);
    }
}
