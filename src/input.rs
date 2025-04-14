use minifb::{Key, MouseButton, MouseMode};

pub struct InputHandler {
    points: Vec<(f64, f64)>,
    mouse_down: bool,
    mouse_pos: (f64, f64),
    is_animating: bool,
    should_close: bool,
    dragging_point: Option<usize>,
    message: Option<String>,
    key_cooldown: u32,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            mouse_down: false,
            mouse_pos: (0.0, 0.0),
            is_animating: false,
            should_close: false,
            dragging_point: None,
            message: None,
            key_cooldown: 0,
        }
    }

    pub fn handle_input(&mut self, window: &mut minifb::Window) {
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Pass) {
            self.mouse_pos = (x as f64, y as f64);
        }

        let was_mouse_down = self.mouse_down;
        self.mouse_down = window.get_mouse_down(MouseButton::Left);

        // Decrement cooldown timer
        if self.key_cooldown > 0 {
            self.key_cooldown -= 1;
        }

        // Handle key presses
        if window.is_key_down(Key::Escape) {
            self.should_close = true;
        } else if window.is_key_down(Key::Enter) && self.key_cooldown == 0 {
            if self.points.is_empty() {
                self.message = Some("Please draw control points first!\nPress Enter to continue".to_string());
            } else if !self.is_animating {
                self.is_animating = true;
            }
            self.key_cooldown = 20; 
        } else if window.is_key_down(Key::Space) && self.key_cooldown == 0 {
            self.points.clear();
            self.is_animating = false;
            self.key_cooldown = 20; 
        }

        // Handle point dragging
        if self.mouse_down {
            if let Some(idx) = self.dragging_point {
                self.points[idx] = self.mouse_pos;
            } else if was_mouse_down {
                if let Some(idx) = self.find_nearest_point() {
                    self.dragging_point = Some(idx);
                    self.points[idx] = self.mouse_pos;
                }
            } else if !self.is_animating {
                // Add a new point
                self.add_point(self.mouse_pos.0, self.mouse_pos.1);
            }
        } else {
            // Mouse released
            self.dragging_point = None;
        }

        if window.is_key_down(Key::Enter) && self.message.is_some() && self.key_cooldown == 0 {
            self.message = None;
            self.key_cooldown = 20;
        }
    }

    fn find_nearest_point(&self) -> Option<usize> {
        let mut closest_idx = None;
        let mut closest_dist = f64::MAX;
        let (mx, my) = self.mouse_pos;
        let threshold = 20.0;

        for (i, &(x, y)) in self.points.iter().enumerate() {
            let dx = x - mx;
            let dy = y - my;
            let dist = (dx * dx + dy * dy).sqrt();
            
            if dist < closest_dist && dist < threshold {
                closest_idx = Some(i);
                closest_dist = dist;
            }
        }

        closest_idx
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push((x, y));
    }

    pub fn points(&self) -> &[(f64, f64)] {
        &self.points
    }

    pub fn is_animating(&self) -> bool {
        self.is_animating
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn dragging_point(&self) -> Option<usize> {
        self.dragging_point
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn clear_message(&mut self) {
        self.message = None;
    }
}


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
