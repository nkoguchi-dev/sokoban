use crate::domain::map::Map;

pub struct Game {
    pub map: Map,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        Game {
            map: Map::new(width, height),
        }
    }
}
