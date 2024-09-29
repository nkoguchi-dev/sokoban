use crate::domain::map::{Direction, MapPosition};

pub trait MovableEntity {
    fn move_entity(&self, direction: Direction) -> Self;
}

// プレイヤーを表す構造体
pub struct Player {
    pub position: MapPosition,
}

impl Player {
    pub fn new(x: u32, y: u32) -> Self {
        Player {
            position: MapPosition { x, y },
        }
    }
}

impl MovableEntity for Player {
    fn move_entity(&self, direction: Direction) -> Player {
        let new_position = self.position.r#move(&direction);
        println!("Player moves to ({}, {})", new_position.x, new_position.y);
        Player {
            position: new_position,
        }
    }
}
