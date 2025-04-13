use minifb::{Window, WindowOptions};
use super::point::Point;
use super::chaikin::Chaikin;
use super::input::InputHandler;

pub struct Canvas {
    window: Window,
    buffer: Vec<u32>,
    chaikin: Chaikin,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        println!("Creating window with dimensions {}x{}", width, height);
        let window = Window::new(
            "Chaikin's Algorithm Animation",
            width,
            height,
            WindowOptions {
                resize: false,
                borderless: true,
                ..Default::default()
            },
        ).expect("Failed to create window");

        let buffer = vec![0; width * height];
        let chaikin = Chaikin::new(Vec::new());
        
        println!("Window created successfully");
        
        Self {
            window,
            buffer,
            chaikin,
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn update(&mut self, input: &mut InputHandler) -> Result<(), String> {
        // Handle input
        input.handle_input(&mut self.window);

        // Clear buffer
        self.buffer.fill(0);

        // Convert points to Point structs
        let points: Vec<Point> = input
            .points()
            .iter()
            .map(|&(x, y)| Point::new(x as f64, y as f64))
            .collect();

        // Update Chaikin points if animating
        if input.is_animating() && !points.is_empty() {
            self.chaikin.set_points(points.clone());
            let new_points = self.chaikin.step();
            self.draw_smooth_curve(&new_points);
        } else {
            self.chaikin.set_points(points.clone());
            self.draw_points(&points);
        }

        // Draw dragging indicator if dragging
        if let Some(idx) = input.dragging_point() {
            let (x, y) = input.points()[idx];
            self.draw_point(x as f64, y as f64, [255, 0, 0], 8.0); // Red circle for dragging point
        }

        // Draw message if present
        if let Some(message) = input.message() {
            self.draw_message(message);
            
            // Clear message after Enter is pressed
            if self.window.is_key_down(minifb::Key::Enter) {
                input.clear_message();
            }
        }

        // Update window
        self.window
            .update_with_buffer(&self.buffer, 800, 600)
            .map_err(|e| format!("Failed to update window: {}", e))
    }

    fn draw_message(&mut self, message: &str) {
        let height = self.window.get_size().1;
        let mut x = 10;
        let y = height - 30;
        
        for c in message.chars() {
            if c == ' ' {
                x += 10;
            } else {
                self.draw_point(x as f64, y as f64, [255, 255, 255], 5.0);
                x += 10;
            }
        }
    }

    fn draw_point(&mut self, x: f64, y: f64, color: [u8; 3], radius: f64) {
        // Convert to buffer coordinates
        let x = x as i32;
        let y = y as i32;

        // Draw circle
        for dx in -radius as i32..=radius as i32 {
            for dy in -radius as i32..=radius as i32 {
                if dx * dx + dy * dy <= (radius * radius) as i32 {
                    let px = x + dx;
                    let py = y + dy;
                    if px >= 0 && px < 800 as i32 && py >= 0 && py < 600 as i32 {
                        let idx = (py * 800 as i32 + px) as usize;
                        self.buffer[idx] = ((color[0] as u32) << 16) | 
                                         ((color[1] as u32) << 8) | 
                                          (color[2] as u32);
                    }
                }
            }
        }
    }

    fn draw_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: [u8; 3]) {
        // Convert to buffer coordinates
        let x1 = x1 as i32;
        let y1 = y1 as i32;
        let x2 = x2 as i32;
        let y2 = y2 as i32;

        // Bresenham's line algorithm
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;
        let mut x = x1;
        let mut y = y1;

        while x != x2 || y != y2 {
            if x >= 0 && x < 800 as i32 && y >= 0 && y < 600 as i32 {
                let idx = (y * 800 as i32 + x) as usize;
                self.buffer[idx] = ((color[0] as u32) << 16) | 
                                 ((color[1] as u32) << 8) | 
                                  (color[2] as u32);
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn draw_smooth_curve(&mut self, points: &[Point]) {
        // Draw control points
        for point in points {
            self.draw_point(
                point.position.x,
                point.position.y,
                point.color,
                if points.len() == 1 { 5.0 } else { 3.0 },
            );
        }

        // Draw lines between points
        for i in 0..points.len() - 1 {
            self.draw_line(
                points[i].position.x,
                points[i].position.y,
                points[i + 1].position.x,
                points[i + 1].position.y,
                points[i].color,
            );
        }
    }

    fn draw_points(&mut self, points: &[Point]) {
        for point in points {
            self.draw_point(
                point.position.x,
                point.position.y,
                point.color,
                if points.len() == 1 { 5.0 } else { 3.0 },
            );
        }
    }
}
