use crate::domain::map::Map;
use std::fmt;

pub struct Game {
    pub map: Map,
}

impl Game {
    pub fn new(width: u16, height: u16) -> Self {
        Game {
            map: Map::new(width, height),
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.map)
    }
}
