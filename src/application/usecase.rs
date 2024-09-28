pub struct GameUseCase;

pub enum InputCommands {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl GameUseCase {
    pub fn new() -> Self {
        GameUseCase
    }

    pub fn move_character(&mut self, command: InputCommands) -> Result<String, String> {
        match command {
            InputCommands::UP => Ok("up".to_string()),
            InputCommands::DOWN => Ok("down".to_string()),
            InputCommands::LEFT => Ok("left".to_string()),
            InputCommands::RIGHT => Ok("right".to_string()),
        }
    }

    pub fn quit_game(&mut self) {}
}
