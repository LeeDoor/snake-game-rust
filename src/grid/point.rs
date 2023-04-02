use std::ops;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl ops::Add for Point {
    type Output = Point;
    fn add (self, sec: Point) ->Point {
        Self {
            x: self.x + sec.x,
            y: self.y + sec.y 
        }
    }
}

impl<'a, 'b> ops::Add<&'a Point> for &'b Point {
    type Output = Point;
    fn add (self, sec: &Point) ->Self::Output {
        Point {
            x: self.x + sec.x,
            y: self.y + sec.y 
        }
    }
}
