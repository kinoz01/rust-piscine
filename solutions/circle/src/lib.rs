use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, p2: Point) -> f64 {
        let dx = self.0 - p2.0;
        let dy = self.1 - p2.1;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Self { center: Point(x, y), radius }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    pub fn intersect(&self, c2: Circle) -> bool {
        let d = self.center.distance(c2.center);
        d >= (self.radius - c2.radius).abs() && d <= self.radius + c2.radius
    }
}
