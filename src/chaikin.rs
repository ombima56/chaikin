use super::point::Point;
use nalgebra::Vector2;
use std::time::{Duration, Instant};

pub struct Chaikin {
    original_points: Vec<Point>,
    control_points: Vec<Point>,
    animated_points: Vec<Point>,
    current_step: usize,
    max_steps: usize,
    animation_timer: Instant,
    animation_duration: Duration,
    t: f64, // Animation progress from 0.0 to 1.0
}

impl Chaikin {
    pub fn new(points: Vec<Point>) -> Self {
        Self {
            original_points: points.clone(),
            control_points: points.clone(),
            animated_points: Vec::new(),
            current_step: 0,
            max_steps: 7,
            animation_timer: Instant::now(),
            animation_duration: Duration::from_millis(500), // Half a second per step
            t: 0.0,
        }
    }

    pub fn step(&mut self) -> Vec<Point> {
        if self.original_points.len() < 2 {
            return self.original_points.clone();
        }

        // Update animation timer
        let elapsed = self.animation_timer.elapsed();
        self.t = elapsed.as_secs_f64() / self.animation_duration.as_secs_f64();

        // If current animation step is complete
        if self.t >= 1.0 {
            // Advance to next step
            self.current_step = (self.current_step + 1) % self.max_steps;
            
            // If we've completed all steps, prepare for next cycle
            if self.current_step == 0 {
                self.control_points = self.original_points.clone();
            } else {
                // Apply one step of Chaikin's algorithm to the control points
                self.control_points = self.apply_chaikin_once(&self.control_points);
            }
            
            // Reset animation timer and progress
            self.animation_timer = Instant::now();
            self.t = 0.0;
        }

        // Calculate the next set of points for animation
        let next_points = if self.current_step == self.max_steps - 1 {
            // If this is the last step, we'll animate back to the original points
            self.original_points.clone()
        } else {
            // Otherwise, calculate the next step of Chaikin's algorithm
            self.apply_chaikin_once(&self.control_points)
        };

        // Interpolate between current control points and next points
        self.animated_points = self.interpolate(&self.control_points, &next_points, self.t);
        
        // Create visualization
        self.create_visualization()
    }
    
    // Apply one iteration of Chaikin's algorithm
    fn apply_chaikin_once(&self, points: &[Point]) -> Vec<Point> {
        if points.len() < 2 {
            return points.to_vec();
        }
        
        let mut new_points = Vec::new();
        let n = points.len();
        
        // Keep first point for open curves
        new_points.push(points[0].clone());
        
        // Apply algorithm to each pair of points
        for i in 0..n-1 {
            let p1 = points[i].position;
            let p2 = points[i+1].position;
            
            // Q point (1/4 point) - 75% of p1, 25% of p2
            let q = Point {
                position: Vector2::new(
                    0.75 * p1.x + 0.25 * p2.x,
                    0.75 * p1.y + 0.25 * p2.y
                ),
                color: [255, 255, 255],
            };
            
            // R point (3/4 point) - 25% of p1, 75% of p2
            let r = Point {
                position: Vector2::new(
                    0.25 * p1.x + 0.75 * p2.x,
                    0.25 * p1.y + 0.75 * p2.y
                ),
                color: [255, 255, 255],
            };
            
            new_points.push(q);
            new_points.push(r);
        }
        
        // Keep last point for open curves
        new_points.push(points[n-1].clone());
        
        new_points
    }
    
    // Interpolate between two sets of points
    fn interpolate(&self, from_points: &[Point], to_points: &[Point], t: f64) -> Vec<Point> {
        // If the number of points differs, we need to handle this specially
        if from_points.len() != to_points.len() {
            // If we're transitioning to more points (normal step)
            if from_points.len() < to_points.len() {
                let mut result = Vec::new();
                
                // Keep original points in their positions
                for point in from_points {
                    result.push(point.clone());
                }
                
                // Gradually introduce new points
                for i in from_points.len()..to_points.len() {
                    // Find nearest point in from_points
                    let new_point = to_points[i];
                    let mut nearest_idx = 0;
                    let mut min_dist = f64::MAX;
                    
                    for (j, from_point) in from_points.iter().enumerate() {
                        let dx = from_point.position.x - new_point.position.x;
                        let dy = from_point.position.y - new_point.position.y;
                        let dist = dx*dx + dy*dy;
                        
                        if dist < min_dist {
                            min_dist = dist;
                            nearest_idx = j;
                        }
                    }
                    
                    // Interpolate from nearest point
                    let from = from_points[nearest_idx].position;
                    let to = new_point.position;
                    
                    let x = from.x + (to.x - from.x) * t;
                    let y = from.y + (to.y - from.y) * t;
                    
                    result.push(Point::with_color(x, y, [255, 255, 255]));
                }
                
                return result;
            } else {
                // If we're transitioning to fewer points (reset to original)
                let mut result = Vec::new();
                
                // Include all target points
                for point in to_points {
                    result.push(point.clone());
                }
                
                // Gradually fade out extra points
                for i in to_points.len()..from_points.len() {
                    // Find nearest point in to_points
                    let old_point = from_points[i];
                    let mut nearest_idx = 0;
                    let mut min_dist = f64::MAX;
                    
                    for (j, to_point) in to_points.iter().enumerate() {
                        let dx = to_point.position.x - old_point.position.x;
                        let dy = to_point.position.y - old_point.position.y;
                        let dist = dx*dx + dy*dy;
                        
                        if dist < min_dist {
                            min_dist = dist;
                            nearest_idx = j;
                        }
                    }
                    
                    // Interpolate towards nearest point
                    let from = old_point.position;
                    let to = to_points[nearest_idx].position;
                    
                    let x = from.x + (to.x - from.x) * t;
                    let y = from.y + (to.y - from.y) * t;
                    
                    // Fade out by reducing alpha (not directly supported,
                    // but we can adjust color brightness)
                    let brightness = ((1.0 - t) * 255.0) as u8;
                    result.push(Point::with_color(x, y, [brightness, brightness, brightness]));
                }
                
                return result;
            }
        }
        
        // Simple case: same number of points
        let mut result = Vec::new();
        
        for i in 0..from_points.len() {
            let from = from_points[i].position;
            let to = to_points[i].position;
            
            let x = from.x + (to.x - from.x) * t;
            let y = from.y + (to.y - from.y) * t;
            
            result.push(Point::with_color(x, y, [255, 255, 255]));
        }
        
        result
    }
    
    // Create visualization of the current state
    fn create_visualization(&self) -> Vec<Point> {
        let mut result = Vec::new();
        
        // Add original control points in red
        for point in &self.original_points {
            result.push(Point::with_color(
                point.position.x,
                point.position.y,
                [255, 0, 0]
            ));
        }
        
        // Add current animated curve points in green
        for point in &self.animated_points {
            result.push(Point::with_color(
                point.position.x,
                point.position.y,
                [0, 255, 0]
            ));
        }
        
        result
    }

    pub fn set_points(&mut self, points: Vec<Point>) {
        self.original_points = points.clone();
        self.control_points = points.clone();
        self.animated_points = Vec::new();
        self.current_step = 0;
        self.animation_timer = Instant::now();
        self.t = 0.0;
    }
}