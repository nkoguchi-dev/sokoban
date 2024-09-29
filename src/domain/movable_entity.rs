//! 移動可能なエンティティを表すモデルを定義します
use crate::domain::map::{Direction, MapPosition};

pub trait MovableEntity {
    /// MovableEntityを移動します
    /// # arguments
    /// - direction: 移動する方向
    fn move_entity(&self, direction: Direction) -> Self;
}

/// プレイヤーを表す構造体
pub struct Player {
    pub position: MapPosition,
}

impl Player {
    /// プレイヤーモデルのインスタンスを作成します
    /// # arguments
    /// - x: 初期位置のx座標
    /// - y: 初期位置のy座標
    ///
    /// TODO: MapPositionを渡した方が良さそう
    pub fn new(x: u32, y: u32) -> Self {
        Player {
            position: MapPosition { x, y },
        }
    }
}

impl MovableEntity for Player {
    fn move_entity(&self, direction: Direction) -> Player {
        Player {
            position: self.position.r#move(&direction),
        }
    }
}
