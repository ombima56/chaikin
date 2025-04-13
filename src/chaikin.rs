use super::point::Point;

pub struct Chaikin {
    points: Vec<Point>,
    current_step: usize,
    max_steps: usize,
    resolution: usize,
}

impl Chaikin {
    pub fn new(points: Vec<Point>) -> Self {
        Self {
            points,
            current_step: 0,
            max_steps: 7,
            resolution: 10, // Number of segments between points
        }
    }

    fn catmull_rom(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), p3: (f64, f64), t: f64) -> (f64, f64) {
        let t2 = t * t;
        let t3 = t2 * t;
        (
            0.5 * (
                (2.0 * p1.0) +
                (-p0.0 + p2.0) * t +
                (2.0 * p0.0 - 5.0 * p1.0 + 4.0 * p2.0 - p3.0) * t2 +
                (-p0.0 + 3.0 * p1.0 - 3.0 * p2.0 + p3.0) * t3
            ),
            0.5 * (
                (2.0 * p1.1) +
                (-p0.1 + p2.1) * t +
                (2.0 * p0.1 - 5.0 * p1.1 + 4.0 * p2.1 - p3.1) * t2 +
                (-p0.1 + 3.0 * p1.1 - 3.0 * p2.1 + p3.1) * t3
            )
        )
    }

    pub fn step(&mut self) -> Vec<Point> {
        if self.points.len() < 2 {
            return self.points.clone();
        }

        let mut new_points = Vec::new();
        let n = self.points.len();

        // Add control points
        new_points.push(self.points[0]);

        // Calculate intermediate points using Catmull-Rom spline
        for i in 0..n - 1 {
            let p1 = self.points[i].position;
            let p2 = self.points[i + 1].position;

            // Add control points
            new_points.push(Point::with_color(p1.x, p1.y, [255, 0, 0]));
            new_points.push(Point::with_color(p2.x, p2.y, [255, 0, 0]));

            // Calculate intermediate points
            for j in 1..self.resolution {
                let t = j as f64 / self.resolution as f64;
                let (x, y) = Self::catmull_rom(
                    (p1.x, p1.y),
                    (p2.x, p2.y),
                    if i + 1 < n - 1 { (self.points[i + 2].position.x, self.points[i + 2].position.y) } else { (p2.x, p2.y) },
                    if i > 0 { (self.points[i - 1].position.x, self.points[i - 1].position.y) } else { (p1.x, p1.y) },
                    t,
                );
                new_points.push(Point::with_color(x, y, [0, 255, 0]));
            }
        }

        self.current_step = (self.current_step + 1) % self.max_steps;
        new_points
    }

    pub fn reset(&mut self) {
        self.current_step = 0;
    }

    pub fn set_points(&mut self, points: Vec<Point>) {
        self.points = points;
        self.reset();
    }
}
