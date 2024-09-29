use crate::domain::map::{Direction, Map, SquareType};
use crate::domain::movable_entity::{MovableEntity, Player};

pub struct Game {
    pub map: Map,
    pub player: Player,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        Game {
            map: Map::new(width, height),
            player: Player::new(1, 1),
        }
    }

    pub fn move_player(self, direction: Direction) -> Self {
        Game {
            map: self.map,
            player: self.player.move_entity(direction),
        }
    }
}

pub struct GameDomainService;

impl GameDomainService {
    pub fn can_move(map: &Map, player: &Player, direction: &Direction) -> bool {
        let new_position = player.position.r#move(direction);
        match map.get_square_type(&new_position) {
            None => false, // NoneはMap外
            Some(square_type) => match square_type {
                SquareType::WALL => false, // 壁に移動することはできない
                SquareType::FLOOR => true,
            },
        }
    }
}
