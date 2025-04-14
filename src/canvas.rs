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
    empty_points_message: Option<(String, Instant)>,
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
            frame_duration: Duration::from_millis(16),
            empty_points_message: None,
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
    
        input.handle_input(&mut self.window);
    
        self.buffer.fill(0);
    
        // Convert points to Point structs
        let points: Vec<Point> = input
            .points()
            .iter()
            .map(|&(x, y)| Point::new(x, y))
            .collect();
    
        // Check if Enter is pressed with no points
        if self.window.is_key_down(minifb::Key::Enter) && points.is_empty() {
            self.empty_points_message = Some((
                "Please draw some points before pressing Enter".to_string(),
                Instant::now()
            ));
        }
    
        // Update Chaikin points if animating
        if input.is_animating() && !points.is_empty() {
            self.chaikin.set_points(points.clone());
            let animated_points = self.chaikin.step();
            self.draw_animated_curve(&animated_points);
        } else {
            self.draw_points(&points);
        }
    
        if let Some(idx) = input.dragging_point() {
            let (x, y) = input.points()[idx];
            self.draw_point(x as f64, y as f64, [255, 0, 0], 8.0);
        }
    
        // Store the message in a local variable to avoid borrowing issues
        let input_message = input.message().map(|s| s.to_string());
        if let Some(message) = input_message {
            self.draw_message(&message);
            
            if self.window.is_key_down(minifb::Key::Enter) {
                input.clear_message();
            }
        }
    
        // Store the empty points message in a local variable to avoid borrowing issues
        let empty_message = self.empty_points_message.as_ref().map(|(msg, time)| (msg.clone(), *time));
        if let Some((message, display_time)) = empty_message {
            let elapsed = Instant::now() - display_time;
            if elapsed < Duration::from_secs(2) {
                self.draw_message(&message);
            } else {
                self.empty_points_message = None;
            }
        }
    
        self.window
            .update_with_buffer(&self.buffer, 800, 600)
            .map_err(|e| format!("Failed to update window: {}", e))
    }

    fn draw_message(&mut self, message: &str) {
        // Calculate dimensions for background
        let width = self.window.get_size().0;
        let height = self.window.get_size().1;
        let lines: Vec<&str> = message.lines().collect();
        let line_count = lines.len();
        
        // Find the longest line to determine background width
        let max_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);
        let bg_width = (max_len * 10).min(width - 20);
        let bg_height = line_count * 25 + 20;
        
        // Draw semi-transparent background rectangle
        let bg_y = height - bg_height - 10;
        for y in bg_y..bg_y + bg_height {
            for x in 5..5 + bg_width {
                if x < width && y < height {
                    let idx = (y * width + x) as usize;
                    if idx < self.buffer.len() {
                        self.buffer[idx] = 0x202020;
                    }
                }
            }
        }
        
        // Draw a border around the rectangle
        let border_color = 0xFFFFFF;
        
        // Top border
        for x in 5..5 + bg_width {
            let idx = (bg_y * width + x) as usize;
            if idx < self.buffer.len() {
                self.buffer[idx] = border_color;
            }
        }
        
        // Bottom border
        for x in 5..5 + bg_width {
            let idx = ((bg_y + bg_height - 1) * width + x) as usize;
            if idx < self.buffer.len() {
                self.buffer[idx] = border_color;
            }
        }
        
        // Left border
        for y in bg_y..bg_y + bg_height {
            let idx = (y * width + 5) as usize;
            if idx < self.buffer.len() {
                self.buffer[idx] = border_color;
            }
        }
        
        // Right border
        for y in bg_y..bg_y + bg_height {
            let idx = (y * width + 5 + bg_width - 1) as usize;
            if idx < self.buffer.len() {
                self.buffer[idx] = border_color;
            }
        }
        
        // Draw the text
        let mut y = bg_y + 15;
        
        for line in lines {
            self.draw_text_string(10, y, line, 0xFFFFFF);
            y += 25; 
        }
    }

    fn draw_text_string(&mut self, x: usize, y: usize, text: &str, color: u32) {
        let mut pos_x = x;
        
        for c in text.chars() {
            self.draw_large_char(pos_x, y, c, color);
            pos_x += 10; 
        }
    }
    
    fn draw_large_char(&mut self, x: usize, y: usize, c: char, color: u32) {
        // Simplified larger characters using blocks instead of bitmaps
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '.' | ',' | '!' | '?' | ':' | ';' | '(' | ')' | ' ' => {
                self.draw_block_char(x, y, c, color);
            },
            _ => {
                self.draw_block_char(x, y, '#', color);
            }
        }
    }

    fn draw_block_char(&mut self, x_start: usize, y_start: usize, c: char, color: u32) {
        let width = self.window.get_size().0;
        
        // Define character shapes using simple block patterns
        // Each character is 8x12 pixels
        
        // Get the pattern for this character
        let pattern = match c {
            'P' => [
                "  ████  ",
                " ██  ██ ",
                " ██  ██ ",
                " █████  ",
                " ██     ",
                " ██     ",
                "        ",
            ],
            'l' => [
                " ██     ",
                " ██     ",
                " ██     ",
                " ██     ",
                " ██     ",
                " ██████ ",
                "        ",
            ],
            'e' => [
                "        ",
                "  ████  ",
                " ██  ██ ",
                " ██████ ",
                " ██     ",
                "  ████  ",
                "        ",
            ],
            'a' => [
                "        ",
                "  ████  ",
                "     ██ ",
                "  █████ ",
                " ██  ██ ",
                "  █████ ",
                "        ",
            ],
            's' => [
                "        ",
                "  ████  ",
                " ██     ",
                "  ████  ",
                "     ██ ",
                " █████  ",
                "        ",
            ],
            'd' => [
                "     ██ ",
                "     ██ ",
                "  █████ ",
                " ██  ██ ",
                " ██  ██ ",
                "  █████ ",
                "        ",
            ],
            'r' => [
                "        ",
                " ██ ██  ",
                " ███    ",
                " ██     ",
                " ██     ",
                " ██     ",
                "        ",
            ],
            'w' => [
                "        ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                " ██████ ",
                "  ██ ██ ",
                "        ",
            ],
            'c' => [
                "        ",
                "  ████  ",
                " ██     ",
                " ██     ",
                " ██     ",
                "  ████  ",
                "        ",
            ],
            'o' => [
                "        ",
                "  ████  ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                "  ████  ",
                "        ",
            ],
            'n' => [
                "        ",
                " ██ ██  ",
                " ███ ██ ",
                " ██ ███ ",
                " ██  ██ ",
                " ██  ██ ",
                "        ",
            ],
            't' => [
                "  ██    ",
                "  ██    ",
                " █████  ",
                "  ██    ",
                "  ██    ",
                "   ███  ",
                "        ",
            ],
            'i' => [
                "  ██    ",
                "        ",
                " ███    ",
                "  ██    ",
                "  ██    ",
                " ████   ",
                "        ",
            ],
            'u' => [
                "        ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                "  █████ ",
                "        ",
            ],
            'E' => [
                " ██████ ",
                " ██     ",
                " █████  ",
                " ██     ",
                " ██     ",
                " ██████ ",
                "        ",
            ],
            'f' => [
                "   ███  ",
                "  ██    ",
                " █████  ",
                "  ██    ",
                "  ██    ",
                "  ██    ",
                "        ",
            ],
            'p' => [
                "        ",
                " █████  ",
                " ██  ██ ",
                " █████  ",
                " ██     ",
                " ██     ",
                "        ",
            ],
            'y' => [
                "        ",
                " ██  ██ ",
                " ██  ██ ",
                "  █████ ",
                "     ██ ",
                " █████  ",
                "        ",
            ],
            'b' => [
                " ██     ",
                " ██     ",
                " █████  ",
                " ██  ██ ",
                " ██  ██ ",
                " █████  ",
                "        ",
            ],
            'g' => [
                "        ",
                "  █████ ",
                " ██  ██ ",
                "  █████ ",
                "     ██ ",
                " █████  ",
                "        ",
            ],
            'h' => [
                " ██     ",
                " ██     ",
                " █████  ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                "        ",
            ],
            'j' => [
                "    ██  ",
                "        ",
                "   ███  ",
                "    ██  ",
                "    ██  ",
                " ████   ",
                "        ",
            ],
            'k' => [
                " ██     ",
                " ██  ██ ",
                " ██ ██  ",
                " ████   ",
                " ██ ██  ",
                " ██  ██ ",
                "        ",
            ],
            'm' => [
                "        ",
                " ██ ██  ",
                " ██████ ",
                " ██ ███ ",
                " ██  ██ ",
                " ██  ██ ",
                "        ",
            ],
            'q' => [
                "        ",
                "  █████ ",
                " ██  ██ ",
                "  █████ ",
                "     ██ ",
                "     ██ ",
                "        ",
            ],
            'v' => [
                "        ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                "  ████  ",
                "   ██   ",
                "        ",
            ],
            'x' => [
                "        ",
                " ██  ██ ",
                "  ████  ",
                "   ██   ",
                "  ████  ",
                " ██  ██ ",
                "        ",
            ],
            'z' => [
                "        ",
                " ██████ ",
                "    ██  ",
                "   ██   ",
                "  ██    ",
                " ██████ ",
                "        ",
            ],
            'A' => [
                "  ████  ",
                " ██  ██ ",
                " ██  ██ ",
                " ██████ ",
                " ██  ██ ",
                " ██  ██ ",
                "        ",
            ],
            'B' => [
                " █████  ",
                " ██  ██ ",
                " █████  ",
                " ██  ██ ",
                " ██  ██ ",
                " █████  ",
                "        ",
            ],
            'C' => [
                "  ████  ",
                " ██  ██ ",
                " ██     ",
                " ██     ",
                " ██  ██ ",
                "  ████  ",
                "        ",
            ],
            'D' => [
                " █████  ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                " █████  ",
                "        ",
            ],
            'F' => [
                " ██████ ",
                " ██     ",
                " █████  ",
                " ██     ",
                " ██     ",
                " ██     ",
                "        ",
            ],
            'G' => [
                "  ████  ",
                " ██  ██ ",
                " ██     ",
                " ██ ███ ",
                " ██  ██ ",
                "  █████ ",
                "        ",
            ],
            'H' => [
                " ██  ██ ",
                " ██  ██ ",
                " ██████ ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                "        ",
            ],
            'I' => [
                " ██████ ",
                "   ██   ",
                "   ██   ",
                "   ██   ",
                "   ██   ",
                " ██████ ",
                "        ",
            ],
            'J' => [
                "     ██ ",
                "     ██ ",
                "     ██ ",
                "     ██ ",
                " ██  ██ ",
                "  ████  ",
                "        ",
            ],
            'K' => [
                " ██  ██ ",
                " ██ ██  ",
                " ████   ",
                " ████   ",
                " ██ ██  ",
                " ██  ██ ",
                "        ",
            ],
            'L' => [
                " ██     ",
                " ██     ",
                " ██     ",
                " ██     ",
                " ██     ",
                " ██████ ",
                "        ",
            ],
            'M' => [
                " ██   ██",
                " ███ ███",
                " ███████",
                " ██ █ ██",
                " ██   ██",
                " ██   ██",
                "        ",
            ],
            'N' => [
                " ██  ██ ",
                " ███ ██ ",
                " ██████ ",
                " ██████ ",
                " ██ ███ ",
                " ██  ██ ",
                "        ",
            ],
            'O' => [
                "  ████  ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                "  ████  ",
                "        ",
            ],
            'Q' => [
                "  ████  ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                " ██ ██  ",
                "  ██ ██ ",
                "        ",
            ],
            'R' => [
                " █████  ",
                " ██  ██ ",
                " █████  ",
                " ████   ",
                " ██ ██  ",
                " ██  ██ ",
                "        ",
            ],
            'S' => [
                "  █████ ",
                " ██     ",
                "  ████  ",
                "     ██ ",
                " ██  ██ ",
                "  ████  ",
                "        ",
            ],
            'T' => [
                "██████  ",
                "  ██    ",
                "  ██    ",
                "  ██    ",
                "  ██    ",
                "  ██    ",
                "        ",
            ],
            'U' => [
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                "  ████  ",
                "        ",
            ],
            'V' => [
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                "  ████  ",
                "   ██   ",
                "        ",
            ],
            'W' => [
                " ██   ██",
                " ██   ██",
                " ██ █ ██",
                " ███████",
                " ███ ███",
                " ██   ██",
                "        ",
            ],
            'X' => [
                " ██  ██ ",
                " ██  ██ ",
                "  ████  ",
                "  ████  ",
                " ██  ██ ",
                " ██  ██ ",
                "        ",
            ],
            'Y' => [
                " ██  ██ ",
                " ██  ██ ",
                "  ████  ",
                "   ██   ",
                "   ██   ",
                "   ██   ",
                "        ",
            ],
            'Z' => [
                " ██████ ",
                "    ██  ",
                "   ██   ",
                "  ██    ",
                " ██     ",
                " ██████ ",
                "        ",
            ],
            '0' => [
                "  ████  ",
                " ██  ██ ",
                " ██ ███ ",
                " ███ ██ ",
                " ██  ██ ",
                "  ████  ",
                "        ",
            ],
            '1' => [
                "   ██   ",
                "  ███   ",
                " ████   ",
                "   ██   ",
                "   ██   ",
                " ██████ ",
                "        ",
            ],
            '2' => [
                "  ████  ",
                " ██  ██ ",
                "    ██  ",
                "   ██   ",
                "  ██    ",
                " ██████ ",
                "        ",
            ],
            '3' => [
                "  ████  ",
                " ██  ██ ",
                "    ██  ",
                "   ███  ",
                " ██  ██ ",
                "  ████  ",
                "        ",
            ],
            '4' => [
                "    ██  ",
                "   ███  ",
                "  ████  ",
                " ██ ██  ",
                " ██████ ",
                "    ██  ",
                "        ",
            ],
            '5' => [
                " ██████ ",
                " ██     ",
                " █████  ",
                "     ██ ",
                " ██  ██ ",
                "  ████  ",
                "        ",
            ],
            '6' => [
                "  ████  ",
                " ██     ",
                " █████  ",
                " ██  ██ ",
                " ██  ██ ",
                "  ████  ",
                "        ",
            ],
            '7' => [
                " ██████ ",
                "     ██ ",
                "    ██  ",
                "   ██   ",
                "  ██    ",
                " ██     ",
                "        ",
            ],
            '8' => [
                "  ████  ",
                " ██  ██ ",
                "  ████  ",
                "  ████  ",
                " ██  ██ ",
                "  ████  ",
                "        ",
            ],
            '9' => [
                "  ████  ",
                " ██  ██ ",
                " ██  ██ ",
                "  █████ ",
                "     ██ ",
                "  ████  ",
                "        ",
            ],
            ' ' => [
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
            ],
            '.' => [
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "  ██    ",
                "        ",
            ],
            ',' => [
                "        ",
                "        ",
                "        ",
                "        ",
                "  ██    ",
                "  ██    ",
                " ██     ",
            ],
            '!' => [
                "  ██    ",
                "  ██    ",
                "  ██    ",
                "  ██    ",
                "        ",
                "  ██    ",
                "        ",
            ],
            '?' => [
                "  ████  ",
                " ██  ██ ",
                "    ██  ",
                "   ██   ",
                "        ",
                "   ██   ",
                "        ",
            ],
            ':' => [
                "        ",
                "  ██    ",
                "        ",
                "        ",
                "  ██    ",
                "        ",
                "        ",
            ],
            ';' => [
                "        ",
                "  ██    ",
                "        ",
                "        ",
                "  ██    ",
                " ██     ",
                "        ",
            ],
            '(' => [
                "   ██   ",
                "  ██    ",
                " ██     ",
                " ██     ",
                "  ██    ",
                "   ██   ",
                "        ",
            ],
            ')' => [
                " ██     ",
                "  ██    ",
                "   ██   ",
                "   ██   ",
                "  ██    ",
                " ██     ",
                "        ",
            ],
            '\n' => [
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
            ],
            _ => [
                " ██████ ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                " ██  ██ ",
                " ██████ ",
                "        ",
            ],
        };
        
        // Draw the pattern
        for (row_idx, &row) in pattern.iter().enumerate() {
            for (col_idx, c) in row.chars().enumerate() {
                if c == '█' {
                    let px = x_start + col_idx;
                    let py = y_start + row_idx;
                    
                    // Draw larger pixels (2x2)
                    for dy in 0..2 {
                        for dx in 0..2 {
                            let x = px + dx;
                            let y = py + dy;
                            
                            if x < self.window.get_size().0 && y < self.window.get_size().1 {
                                let idx = (y * width + x) as usize;
                                if idx < self.buffer.len() {
                                    self.buffer[idx] = color;
                                }
                            }
                        }
                    }
                }
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
                [255, 165, 0]
            );
        }
    }
}
