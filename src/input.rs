use minifb::{Key, MouseButton, MouseMode};

pub struct InputHandler {
    points: Vec<(f64, f64)>,
    mouse_down: bool,
    mouse_pos: (f64, f64),
    is_animating: bool,
    should_close: bool,
    dragging_point: Option<usize>,
    message: Option<String>,
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
        }
    }

    pub fn handle_input(&mut self, window: &mut minifb::Window) {
        // Update mouse position
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Pass) {
            self.mouse_pos = (x as f64, y as f64);
        }

        // Check if mouse button is down
        self.mouse_down = window.get_mouse_down(MouseButton::Left);

        // Handle key presses
        if window.is_key_down(Key::Escape) {
            self.should_close = true;
        } else if window.is_key_down(Key::Enter) {
            if self.points.is_empty() {
                self.message = Some("Please draw control points first!\nPress Enter to continue".to_string());
            } else {
                self.is_animating = true;
            }
        } else if window.is_key_down(Key::Space) {
            self.points.clear();
            self.is_animating = false;
        }

        // Handle point dragging
        if self.mouse_down {
            if let Some(idx) = self.find_nearest_point() {
                self.dragging_point = Some(idx);
                self.points[idx] = self.mouse_pos;
            } else if !self.is_animating {
                self.add_point(self.mouse_pos.0, self.mouse_pos.1);
            }
        } else {
            self.dragging_point = None;
        }

        // Clear message if Enter is pressed again
        if window.is_key_down(Key::Enter) && self.message.is_some() {
            self.message = None;
        }
    }

    fn find_nearest_point(&self) -> Option<usize> {
        let mut closest_idx = None;
        let mut closest_dist = f64::MAX;
        let (mx, my) = self.mouse_pos;
        let threshold = 20.0; // Distance threshold for dragging

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
