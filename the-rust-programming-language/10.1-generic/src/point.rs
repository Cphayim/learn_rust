pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(self) -> T {
        self.y
    }
}

impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
