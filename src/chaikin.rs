use super::point::Point;
use nalgebra::Vector2;
use std::time::Instant;
pub struct Chaikin {
    original_points: Vec<Point>,
    current_points: Vec<Point>,
    next_points: Vec<Point>,
    animation_progress: f64,
    current_step: usize,
    max_steps: usize,
    last_update: Instant,
    animation_speed: f64,
}

impl Chaikin {
    pub fn new(points: Vec<Point>) -> Self {
        Self {
            original_points: points.clone(),
            current_points: points.clone(),
            next_points: Vec::new(),
            animation_progress: 0.0,
            current_step: 0,
            max_steps: 7,
            last_update: Instant::now(),
            animation_speed: 1.0,
        }
    }

    pub fn step(&mut self) -> Vec<Point> {
        if self.original_points.len() < 2 {
            return self.original_points.clone();
        }

        // Calculate time delta for smooth animation
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f64();
        self.last_update = now;

        // Update animation progress
        self.animation_progress += delta_time * self.animation_speed;
        
        // If we completed the current animation step
        if self.animation_progress >= 1.0 {
            self.animation_progress = 0.0;
            
            // Move to the next iteration
            self.current_step = (self.current_step + 1) % self.max_steps;
            self.current_points = self.next_points.clone();
            self.next_points = Vec::new();
        }
        
        // If we need to calculate the next points
        if self.next_points.is_empty() {
            if self.current_step == 0 {
                // If we're starting, use original points as current
                self.current_points = self.original_points.clone();
                
                // Calculate first Chaikin iteration as next
                if self.original_points.len() >= 2 {
                    self.next_points = self.apply_chaikin(&self.original_points);
                } else {
                    self.next_points = self.original_points.clone();
                }
            } else if self.current_step == self.max_steps - 1 {
                // If we're at the last step, next will be original points again
                self.next_points = self.original_points.clone();
            } else {
                // Otherwise, calculate next Chaikin iteration
                self.next_points = self.apply_chaikin(&self.current_points);
            }
        }
        
        // Interpolate between current and next points
        let result = self.interpolate(self.animation_progress);
        
        // Return visualization
        self.create_visualization(result)
    }
    
    // Apply one step of Chaikin's algorithm
    fn apply_chaikin(&self, points: &[Point]) -> Vec<Point> {
        if points.len() < 2 {
            return points.to_vec();
        }
        
        let mut result = Vec::new();
        
        // First point stays the same (for open curves)
        result.push(points[0].clone());
        
        // Apply Chaikin's corner cutting
        for i in 0..points.len() - 1 {
            let p0 = points[i].position;
            let p1 = points[i+1].position;
            
            // Q point (1/4 point)
            result.push(Point {
                position: Vector2::new(
                    0.75 * p0.x + 0.25 * p1.x,
                    0.75 * p0.y + 0.25 * p1.y
                ),
                color: [255, 255, 255],
            });
            
            // R point (3/4 point)
            result.push(Point {
                position: Vector2::new(
                    0.25 * p0.x + 0.75 * p1.x,
                    0.25 * p0.y + 0.75 * p1.y
                ),
                color: [255, 255, 255],
            });
        }
        
        // Last point stays the same (for open curves)
        result.push(points[points.len() - 1].clone());
        
        result
    }
    
    // Interpolate between current and next points based on animation progress
    fn interpolate(&self, t: f64) -> Vec<Point> {
        // If either set is empty, return the other
        if self.current_points.is_empty() {
            return self.next_points.clone();
        }
        if self.next_points.is_empty() {
            return self.current_points.clone();
        }
        
        // Handle different point counts
        if self.current_points.len() != self.next_points.len() {
            return self.interpolate_different_point_counts(t);
        }
        
        // Simple case: same number of points
        let mut result = Vec::new();
        for i in 0..self.current_points.len() {
            let p1 = self.current_points[i].position;
            let p2 = self.next_points[i].position;
            
            result.push(Point {
                position: Vector2::new(
                    p1.x + t * (p2.x - p1.x),
                    p1.y + t * (p2.y - p1.y)
                ),
                color: self.current_points[i].color,
            });
        }
        
        result
    }
    
    // Handle interpolation when point counts differ
    fn interpolate_different_point_counts(&self, t: f64) -> Vec<Point> {
        let mut result = Vec::new();
        
        // First and last points always stay the same
        result.push(Point {
            position: Vector2::new(
                self.current_points[0].position.x + t * (self.next_points[0].position.x - self.current_points[0].position.x),
                self.current_points[0].position.y + t * (self.next_points[0].position.y - self.current_points[0].position.y)
            ),
            color: [255, 255, 255],
        });
        
        // For intermediate points, we need to map between different counts
        // Use a continuous mapping based on position along the curve
        
        // Create normalized positions for both point sets
        let curr_len = self.current_points.len();
        let next_len = self.next_points.len();
        
        // For each point in the more dense set, interpolate a corresponding point
        if curr_len > next_len {
            for i in 1..curr_len-1 {
                // Normalized position in the curve [0..1]
                let pos = i as f64 / (curr_len - 1) as f64;
                
                // Find two nearest points in next_points
                let next_idx = (pos * (next_len - 1) as f64).floor() as usize;
                let next_idx2 = (next_idx + 1).min(next_len - 1);
                
                let next_pos = next_idx as f64 / (next_len - 1) as f64;
                let next_pos2 = next_idx2 as f64 / (next_len - 1) as f64;
                
                // Local interpolation factor
                let local_t = if next_pos2 > next_pos {
                    (pos - next_pos) / (next_pos2 - next_pos)
                } else {
                    0.0
                };
                
                // Interpolate between the two nearest points
                let p1 = self.next_points[next_idx].position;
                let p2 = self.next_points[next_idx2].position;
                let target = Vector2::new(
                    p1.x + local_t * (p2.x - p1.x),
                    p1.y + local_t * (p2.y - p1.y)
                );
                
                // Interpolate from current to target
                let curr = self.current_points[i].position;
                result.push(Point {
                    position: Vector2::new(
                        curr.x + t * (target.x - curr.x),
                        curr.y + t * (target.y - curr.y)
                    ),
                    color: [255, 255, 255],
                });
            }
        } else {
            for i in 1..next_len-1 {
                // Normalized position in the curve [0..1]
                let pos = i as f64 / (next_len - 1) as f64;
                
                // Find two nearest points in current_points
                let curr_idx = (pos * (curr_len - 1) as f64).floor() as usize;
                let curr_idx2 = (curr_idx + 1).min(curr_len - 1);
                
                let curr_pos = curr_idx as f64 / (curr_len - 1) as f64;
                let curr_pos2 = curr_idx2 as f64 / (curr_len - 1) as f64;
                
                // Local interpolation factor
                let local_t = if curr_pos2 > curr_pos {
                    (pos - curr_pos) / (curr_pos2 - curr_pos)
                } else {
                    0.0
                };
                
                // Interpolate between the two nearest points
                let p1 = self.current_points[curr_idx].position;
                let p2 = self.current_points[curr_idx2].position;
                let source = Vector2::new(
                    p1.x + local_t * (p2.x - p1.x),
                    p1.y + local_t * (p2.y - p1.y)
                );
                
                // Interpolate from source to target
                let target = self.next_points[i].position;
                result.push(Point {
                    position: Vector2::new(
                        source.x + t * (target.x - source.x),
                        source.y + t * (target.y - source.y)
                    ),
                    color: [255, 255, 255],
                });
            }
        }
        
        // Last point
        let last_curr = self.current_points[curr_len - 1].position;
        let last_next = self.next_points[next_len - 1].position;
        result.push(Point {
            position: Vector2::new(
                last_curr.x + t * (last_next.x - last_curr.x),
                last_curr.y + t * (last_next.y - last_curr.y)
            ),
            color: [255, 255, 255],
        });
        
        result
    }
    
    // Create visualization with original control points highlighted
    fn create_visualization(&self, points: Vec<Point>) -> Vec<Point> {
        let mut result = Vec::new();
        
        // Add original control points in red
        for point in &self.original_points {
            result.push(Point::with_color(
                point.position.x,
                point.position.y,
                [255, 0, 0]
            ));
        }
        
        // Add animation points in green
        for point in points {
            result.push(Point::with_color(
                point.position.x,
                point.position.y,
                [0, 255, 0]
            ));
        }
        
        result
    }

    pub fn set_points(&mut self, points: Vec<Point>) {
        if self.original_points != points {
            self.original_points = points.clone();
            self.current_points = points;
            self.next_points = Vec::new();
            self.animation_progress = 0.0;
            self.current_step = 0;
            self.last_update = Instant::now();
        }
    }
}