use rand::Rng;
use raster::{Color, Image};

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, c: Color);
}

pub trait Drawable {
    fn draw(&self, img: &mut Image);
    fn color() -> Color {
        let mut rng = rand::thread_rng();
        Color::rgb(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255))
    }
}

//****************************************** Point ******************************************
#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    col: Color,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y, col: Point::color() }
    }
    pub fn random(w: i32, h: i32) -> Self {
        let mut g = rand::thread_rng();
        Self::new(g.gen_range(0..w), g.gen_range(0..h))
    }
}

impl Drawable for Point {
    fn draw(&self, img: &mut Image) {
        img.display(self.x, self.y, self.col.clone());
    }
}

//****************************************** Line ******************************************
pub struct Line {
    a: Point,
    b: Point,
    col: Color,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a: a, b: b, col: Line::color() }
    }
    pub fn random(w: i32, h: i32) -> Self {
        Self::new(Point::random(w, h), Point::random(w, h))
    }
}

impl Drawable for Line {
    fn draw(&self, img: &mut Image) {
        let (x0, y0) = (self.a.x as f32, self.a.y as f32);
        let (x1, y1) = (self.b.x as f32, self.b.y as f32);

        let dx = x1 - x0;
        let dy = y1 - y0;

        let steps = dx.abs().max(dy.abs());
        if steps == 0.0 {
            img.display(x0 as i32, y0 as i32, self.col.clone());
            return;
        }

        let x_inc = dx / steps;
        let y_inc = dy / steps;

        let (mut x, mut y) = (x0, y0);

        for _ in 0..=steps as i32 {
            // if we don't use clone() here we lose ownership of color to display(), and the next pixel won't find the color.
            img.display(x.round() as i32, y.round() as i32, self.col.clone());
            x += x_inc;
            y += y_inc;
        }
    }
}

//****************************************** Triangle ******************************************
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
    col: Color,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self { a: a.clone(), b: b.clone(), c: c.clone(), col: Triangle::color() }
    }
}

impl Drawable for Triangle {
    fn draw(&self, img: &mut Image) {
        (Line { a: self.a.clone(), b: self.b.clone(), col: self.col.clone() }).draw(img);
        (Line { a: self.b.clone(), b: self.c.clone(), col: self.col.clone() }).draw(img);
        (Line { a: self.c.clone(), b: self.a.clone(), col: self.col.clone() }).draw(img);
    }
}

//****************************************** Rectangle ******************************************
pub struct Rectangle {
    tl: Point,
    br: Point,
    col: Color,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let (x0, y0) = (p1.x.min(p2.x), p1.y.min(p2.y));
        let (x1, y1) = (p1.x.max(p2.x), p1.y.max(p2.y));
        Self { tl: Point::new(x0, y0), br: Point::new(x1, y1), col: Rectangle::color() }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, img: &mut Image) {
        // top & bottom
        for x in self.tl.x..=self.br.x {
            img.display(x, self.tl.y, self.col.clone());
            img.display(x, self.br.y, self.col.clone());
        }
        // sides
        for y in self.tl.y..=self.br.y {
            img.display(self.tl.x, y, self.col.clone());
            img.display(self.br.x, y, self.col.clone());
        }
    }
}

//****************************************** Circle ******************************************
pub struct Circle {
    c: Point,
    r: i32,
    col: Color,
}

impl Circle {
    pub fn new(c: Point, r: i32) -> Self {
        Self { c, r, col: Circle::color() }
    }
    pub fn random(w: i32, h: i32) -> Self {
        let mut g = rand::thread_rng();
        let c = Point::random(w, h);
        let max_r = w.max(h) / 2;
        Self::new(c, g.gen_range(3..=max_r))
    }
}

impl Drawable for Circle {
    fn draw(&self, img: &mut Image) {
        let (mut x, mut y, mut p) = (0, -self.r, -self.r);
        let (cx, cy) = (self.c.x, self.c.y);
        while x < -y {
            if p < 0 {
                p += 2 * x + 1;
            } else {
                y += 1;
                p += 2 * (y + x) + 1;
            }

            for (px, py) in [
                (cx + x, cy + y),
                (cx - x, cy + y),
                (cx + x, cy - y),
                (cx - x, cy - y),
                (cx + y, cy + x),
                (cx + y, cy - x),
                (cx - y, cy + x),
                (cx - y, cy - x),
            ] {
                img.display(px, py, self.col.clone());
            }
            x += 1;
        }
    }
}
