use std::collections::HashMap;

/// マスの位置を表す構造体
/// 8x8のMapの場合x:0 , y:0 が左下でx:7, y:7が右上
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct SquarePosition {
    x: u16,
    y: u16,
}

/// マスを表す構造体
/// あとで荷物と人が表現される予定
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
    blocks: HashMap<SquarePosition, Square>,
}

/// 指定サイズのMapを生成します
pub fn generate_map(width: u16, height: u16) -> Map {
    let blocks = (0..width)
        .flat_map(|x| {
            (0..height).map(move |y| {
                let square_type = if x == 0 || x == (width - 1) || y == 0 || y == (height - 1) {
                    SquareType::WALL
                } else {
                    SquareType::FLOOR
                };
                (SquarePosition { x, y }, Square { square_type })
            })
        })
        .collect();
    Map { blocks }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_map() {
        let left = generate_map(4, 4);
        let blocks: HashMap<_, _> = (0..4)
            .flat_map(|xx| {
                (0..4).map(move |yy| {
                    let square_type = if xx == 0 || xx == 3 || yy == 0 || yy == 3 {
                        SquareType::WALL
                    } else {
                        SquareType::FLOOR
                    };
                    (SquarePosition { x: xx, y: yy }, Square { square_type })
                })
            })
            .collect();
        let right = Map { blocks };
        assert_eq!(left, right);
    }
}
