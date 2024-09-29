use crate::domain::game::Game;
use crate::domain::map::{SquarePosition, SquareType};

pub struct GameUseCase {
    game: Option<Game>,
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
        GameUseCase { game: None }
    }

    pub fn start_game(&mut self, width: u16, height: u16) {
        let game = Game::new(width, height);
        self.game = Some(game);
    }

    pub fn move_character(&mut self, command: InputCommands) -> Result<String, String> {
        match command {
            InputCommands::UP => Ok("up".to_string()),
            InputCommands::DOWN => Ok("down".to_string()),
            InputCommands::LEFT => Ok("left".to_string()),
            InputCommands::RIGHT => Ok("right".to_string()),
        }
    }

    pub fn get_display_model(&self) -> GameStateQueryOutput {
        let game = match &self.game {
            Some(g) => g,
            None => return GameStateQueryOutput { map: vec![] },
        };
        let mut output_map = vec![];

        for y in (0..game.map.height).rev() {
            let mut row = vec![];
            for x in 0..game.map.width {
                let position = SquarePosition { x, y };
                let char = match game.map.get_square_type(&position) {
                    Some(&SquareType::FLOOR) => ' ',
                    Some(&SquareType::WALL) => '*',
                    None => '?', // 該当なしの場合
                };
                row.push(char);
            }
            output_map.push(row);
        }
        GameStateQueryOutput { map: output_map }
    }

    pub fn quit_game(&mut self) {
        self.game = None;
    }
}
