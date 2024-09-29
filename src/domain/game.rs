use crate::domain::map::Map;

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
