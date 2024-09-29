use crate::domain::game::{Game, GameDomainService};
use crate::domain::map::{Direction, MapPosition, SquareType};

pub struct GameUseCase {
    game_status: GameStatus,
}

pub enum GameStatus {
    NotStarted,
    Playing(Game),
    Finished,
}

pub enum InputCommands {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct GameStateQueryOutput {
    pub map: Vec<Vec<char>>,
}

impl GameUseCase {
    pub fn new() -> Self {
        GameUseCase {
            game_status: GameStatus::NotStarted,
        }
    }

    /// ゲームを開始します
    /// # arguments
    /// - width: 作成するMapの幅を指定します
    /// - height: 作成するMapの高さを指定します
    pub fn start_game(&mut self, width: u32, height: u32) {
        let game = Game::new(width, height);
        self.game_status = GameStatus::Playing(game);
    }

    /// プレイヤーを指定方向に動かします
    ///
    /// # arguments
    /// - command: 移動したい方向
    ///
    /// # return value
    /// - Ok: 移動が完了した場合
    /// - Err: ゲームの状態が移動可能ではなかった場合
    pub fn move_character(&mut self, command: InputCommands) -> Result<String, String> {
        let game = match std::mem::replace(&mut self.game_status, GameStatus::NotStarted) {
            GameStatus::NotStarted | GameStatus::Finished => {
                self.game_status = GameStatus::NotStarted; // 元の状態に戻す
                return Err("ゲームが開始していません。".to_string());
            }
            GameStatus::Playing(game) => game,
        };
        let direction = match command {
            InputCommands::UP => Direction::UP,
            InputCommands::DOWN => Direction::DOWN,
            InputCommands::LEFT => Direction::LEFT,
            InputCommands::RIGHT => Direction::RIGHT,
        };
        if !GameDomainService::can_move(&game.map, &game.player, &direction) {
            self.game_status = GameStatus::Playing(game); // 再びPlayingに戻す
            return Ok("その方向には動けません".to_string());
        };
        let new_game = game.move_player(direction);
        self.game_status = GameStatus::Playing(new_game);
        Ok("動いたよ".to_string())
    }

    /// ゲームの状態表示に必要な情報を返却します
    /// # return value
    /// - GameStateQueryOutput: 表示用のゲームの状態を表す構造体
    pub fn get_display_model(&self) -> GameStateQueryOutput {
        let game = match &self.game_status {
            GameStatus::NotStarted | GameStatus::Finished => {
                return GameStateQueryOutput { map: vec![] }
            }
            GameStatus::Playing(game) => game,
        };
        let mut output_map = vec![];

        for y in (0..game.map.height).rev() {
            let mut row = vec![];
            for x in 0..game.map.width {
                let position = MapPosition { x, y };
                let char = match game.map.get_square_type(&position) {
                    None => '?',
                    Some(square_type) => match square_type {
                        SquareType::WALL => '■',
                        SquareType::FLOOR => {
                            if game.player.position == position {
                                'P'
                            } else {
                                ' '
                            }
                        }
                    },
                };
                row.push(char);
            }
            output_map.push(row);
        }
        GameStateQueryOutput { map: output_map }
    }

    /// ゲームを終了します
    pub fn quit_game(&mut self) {
        self.game_status = GameStatus::Finished;
    }
}
