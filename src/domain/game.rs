//! ゲームモデルとサービスを定義します

use crate::domain::map::{Direction, Map, MapPosition, SquareType};
use crate::domain::movable_entity::{Box, MovableEntity, Player};

/// １回のゲームを表すモデル
pub struct Game {
    pub map: Map,
    pub player: Player,
    pub boxes: Vec<Box>,
}

impl Game {
    /// ゲームモデルをインスタンス化します
    /// # arguments
    /// - width: ゲームで利用するMapの幅
    /// - height: ゲームで利用するMapの高さ
    ///
    /// # return value
    /// ゲームモデルのインスタンス
    pub fn new(width: u32, height: u32) -> Self {
        Game {
            map: Map::new(width, height),
            player: Player::new(1, 1),
            boxes: vec![Box::new(MapPosition { x: 3, y: 3 })],
        }
    }

    /// プレイヤーを移動します
    /// # arguments
    /// - direction: 移動する方向
    pub fn move_player(self, direction: Direction) -> Self {
        Game {
            map: self.map,
            player: self.player.move_entity(direction),
            boxes: self.boxes,
        }
    }
}

/// ゲームモデルとその他のモデルが関連するチェックを行うサービスです
pub struct GameDomainService;

impl GameDomainService {
    /// 現在のゲーム状態で指定方向にプレイヤーが移動可能かどうかをチェックします
    /// # arguments
    /// - map: ゲームで利用してるMap
    /// - player: 現在のプレイヤーの状態
    /// - direction: 移動したい方向
    /// # return value
    /// 移動可能な場合はtrueを返却します
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
