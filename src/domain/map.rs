//! Mapモデルを定義します
use std::collections::HashMap;

/// マスの位置を表す構造体
/// 8x8のMapの場合x:0 , y:0 が左下でx:7, y:7が右上
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct MapPosition {
    pub x: u32,
    pub y: u32,
}

impl MapPosition {
    /// 現在の位置から指定方向に移動した場合のPositionを返却します
    /// # arguments
    /// - direction: 取得したい位置の方向
    pub fn r#move(&self, direction: &Direction) -> Self {
        let (x, y) = match direction {
            Direction::UP => (self.x, self.y + 1),
            Direction::DOWN => (self.x, self.y - 1),
            Direction::LEFT => (self.x + 1, self.y),
            Direction::RIGHT => (self.x - 1, self.y),
        };
        MapPosition { x, y }
    }
}

/// 方向を表すモデル
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

/// マスを表す構造体
#[derive(PartialEq, Debug)]
pub struct Square {
    square_type: SquareType,
}

/// マスのタイプを表すenum
#[derive(PartialEq, Debug)]
pub enum SquareType {
    FLOOR,
    WALL,
}

/// Mapを表す構造体
#[derive(PartialEq, Debug)]
pub struct Map {
    /// Mapの幅
    pub width: u32,
    /// Mapの高さ
    pub height: u32,
    /// Mapの要素
    blocks: HashMap<MapPosition, Square>,
}

impl Map {
    /// 指定サイズのMapを生成します
    /// # arugments
    /// - width: 生成するMapの幅
    /// - height: 生成するMapの高さ
    pub fn new(width: u32, height: u32) -> Self {
        Map {
            width: width,
            height: height,
            blocks: generate_blocks(width, height),
        }
    }

    /// 指定位置のマスのタイプを返却します
    /// # arguments
    /// - position: タイプを取得するマスの位置
    /// # return value
    /// Some: 指定位置のMapタイプ
    /// None: 指定位置がMap外の場合
    pub fn get_square_type(&self, position: &MapPosition) -> Option<&SquareType> {
        let square = &self.blocks.get(position);
        match square {
            Some(value) => Some(&value.square_type),
            None => None,
        }
    }
}

/// 指定サイズのMapを生成します
fn generate_blocks(width: u32, height: u32) -> HashMap<MapPosition, Square> {
    (0..width)
        .flat_map(|x| {
            (0..height).map(move |y| {
                let square_type = if x == 0 || x == (width - 1) || y == 0 || y == (height - 1) {
                    SquareType::WALL
                } else {
                    SquareType::FLOOR
                };
                (MapPosition { x, y }, Square { square_type })
            })
        })
        .collect()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     /// 指定サイズのMapを持ったインスタンスを作ることができること
//     fn test_instance() {
//         let left = generate_blocks(4, 4);
//         let blocks: HashMap<_, _> = (0..4)
//             .flat_map(|xx| {
//                 (0..4).map(move |yy| {
//                     let square_type = if xx == 0 || xx == 3 || yy == 0 || yy == 3 {
//                         SquareType::WALL
//                     } else {
//                         SquareType::FLOOR
//                     };
//                     (SquarePosition { x: xx, y: yy }, Square { square_type })
//                 })
//             })
//             .collect();
//         let right = Map::new(4, 4);
//         assert_eq!(left, right);
//     }
// }
