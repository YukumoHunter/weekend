#[derive(Clone, Copy, Debug)]
pub struct Trajectory<T> {
    pub from: T,
    pub to: T,
    pub timespan: Timespan,
}

impl<T> Trajectory<T> {
    pub fn new(from: T, to: T, timespan: Timespan) -> Self {
        Self { from, to, timespan }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Timespan {
    pub start: f32,
    pub end: f32,
}

impl Timespan {
    pub fn new(start: f32, end: f32) -> Self {
        Self { start, end }
    }

    pub fn difference(&self) -> f32 {
        self.end - self.start
    }
}
