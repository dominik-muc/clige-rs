use super::Level;

pub struct Player<'a> {
    location: &'a Level<'a>,
    health: u32,
    score: u32,
}

impl<'a> Player<'a> {
    pub fn new(location: &'a Level) -> Self {
        Self {
            location,
            health: 100,
            score: 0,
        }
    }
    pub fn get_location(&self) -> &Level {
        self.location
    }
}
