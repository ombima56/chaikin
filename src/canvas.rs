use minifb::{Window, WindowOptions};
use std::time::{Duration, Instant};
use super::point::Point;
use super::chaikin::Chaikin;
use super::input::InputHandler;

pub struct Canvas {
    window: Window,
    buffer: Vec<u32>,
    chaikin: Chaikin,
    last_frame_time: Instant,
    frame_duration: Duration,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        println!("Creating window with dimensions {}x{}", width, height);
        let window = Window::new(
            "Chaikin's Algorithm Animation",
            width,
            height,
            WindowOptions {
                resize: true,
                borderless: false,
                title: true,
                scale: minifb::Scale::X1,
                scale_mode: minifb::ScaleMode::AspectRatioStretch,
                topmost: false,
                transparency: false,
                none: false,
            },
        ).expect("Failed to create window");

        let buffer = vec![0; width * height];
        let chaikin = Chaikin::new(Vec::new());
        
        println!("Window created successfully");
        
        Self {
            window,
            buffer,
            chaikin,
            last_frame_time: Instant::now(),
            frame_duration: Duration::from_millis(16), // ~60 FPS
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn update(&mut self, input: &mut InputHandler) -> Result<(), String> {
        // Limit frame rate to avoid consuming too much CPU
        let now = Instant::now();
        let elapsed = now - self.last_frame_time;
        if elapsed < self.frame_duration {
            std::thread::sleep(self.frame_duration - elapsed);
        }
        self.last_frame_time = Instant::now();

        // Handle input
        input.handle_input(&mut self.window);

        // Clear buffer
        self.buffer.fill(0);

        // Convert points to Point structs
        let points: Vec<Point> = input
            .points()
            .iter()
            .map(|&(x, y)| Point::new(x, y))
            .collect();

        // Update Chaikin points if animating
        if input.is_animating() && !points.is_empty() {
            self.chaikin.set_points(points.clone());
            let animated_points = self.chaikin.step();
            self.draw_animated_curve(&animated_points);
        } else {
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
        let mut y = height - 30;
        let lines: Vec<&str> = message.lines().collect();
        
        for line in lines {
            let mut x = 10;
            for c in line.chars() {
                if c == ' ' {
                    x += 10;
                } else {
                    self.draw_char(x, y, c, [255, 255, 255]);
                    x += 10;
                }
            }
            y -= 15;
        }
    }
    
    fn draw_char(&mut self, x: usize, y: usize, c: char, color: [u8; 3]) {
        // Very simple character rendering - just dots for simplicity
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '.' | ',' | '!' | '?' | ':' | ';' | '(' | ')' => {
                self.draw_point(x as f64, y as f64, color, 3.0);
            }
            _ => {}
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
                    if px >= 0 && px < 800 && py >= 0 && py < 600 {
                        let idx = (py * 800 + px) as usize;
                        if idx < self.buffer.len() {
                            self.buffer[idx] = ((color[0] as u32) << 16) | 
                                             ((color[1] as u32) << 8) | 
                                              (color[2] as u32);
                        }
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
            if x >= 0 && x < 800 && y >= 0 && y < 600 {
                let idx = (y * 800 + x) as usize;
                if idx < self.buffer.len() {
                    self.buffer[idx] = ((color[0] as u32) << 16) | 
                                     ((color[1] as u32) << 8) | 
                                      (color[2] as u32);
                }
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

    fn draw_animated_curve(&mut self, points: &[Point]) {
        // Draw all points first
        for point in points {
            let color = point.color;
            let size = if color == [255, 0, 0] { 5.0 } else { 2.5 };
            self.draw_point(point.position.x, point.position.y, color, size);
        }

        // Find and connect points of the same color to form continuous curves
        // let mut red_points = Vec::new();
        let mut green_points = Vec::new();
        
        for point in points {
            match point.color {
                // [255, 0, 0] => red_points.push(point),
                [0, 255, 0] => green_points.push(point),
                _ => {} // Ignore other colors
            }
        }

        // Draw lines between animated curve points
        for i in 0..green_points.len().saturating_sub(1) {
            self.draw_line(
                green_points[i].position.x,
                green_points[i].position.y,
                green_points[i + 1].position.x,
                green_points[i + 1].position.y,
                [0, 255, 255] // Cyan for the animated curve - better contrast
            );
        }
    }

    fn draw_points(&mut self, points: &[Point]) {
        // Draw points
        for point in points {
            // Special case: single point just shows as a larger circle
            // Two points will show as a line with circles at endpoints
            // More points will show control points and the curve
            self.draw_point(
                point.position.x,
                point.position.y,
                [255, 165, 0], // Orange for control points - more visible
                if points.len() == 1 { 6.0 } else { 4.0 }, // Larger circles for better visibility
            );
        }
        
        // Special case: draw line between points when exactly two points
        if points.len() == 2 {
            self.draw_line(
                points[0].position.x,
                points[0].position.y,
                points[1].position.x,
                points[1].position.y,
                [255, 165, 0] // Orange to match control points
            );
        }

    }
}
