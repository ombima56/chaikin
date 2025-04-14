use crate::input::InputHandler;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_initializes_correctly() {
        let handler = InputHandler::new();
        assert!(handler.points().is_empty());
        assert!(!handler.is_animating());
        assert!(!handler.should_close());
        assert_eq!(handler.dragging_point(), None);
        assert_eq!(handler.message(), None);
    }

    #[test]
    fn test_add_point_and_get_points() {
        let mut handler = InputHandler::new();
        handler.add_point(100.0, 200.0);
        assert_eq!(handler.points(), &[(100.0, 200.0)]);
    }

    #[test]
    fn test_clear_message() {
        let mut handler = InputHandler::new();
        handler.add_point(1.0, 1.0); // trigger non-empty points
        handler.clear_message(); // should be safe even if message is None
        assert_eq!(handler.message(), None);
    }

    #[test]
    fn test_find_nearest_point_returns_closest_index() {
        let mut handler = InputHandler::new();
        handler.add_point(10.0, 10.0);
        handler.add_point(50.0, 50.0);
        handler.mouse_pos = (11.0, 11.0); // very close to first point

        let idx = handler.find_nearest_point();
        assert_eq!(idx, Some(0));
    }

    #[test]
    fn test_find_nearest_point_returns_none_if_too_far() {
        let mut handler = InputHandler::new();
        handler.add_point(10.0, 10.0);
        handler.mouse_pos = (100.0, 100.0); // too far

        let idx = handler.find_nearest_point();
        assert_eq!(idx, None);
    }
}
