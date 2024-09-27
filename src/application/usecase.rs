pub struct GameUseCase;

impl GameUseCase {
    pub fn new() -> Self {
        GameUseCase
    }

    pub fn move_character(&mut self, input: &str) -> Result<(), String> {
        match input {
            "w" | "a" | "s" | "d" => {
                println!("input: {}", input);
                Ok(())
            }
            "q" => Err("quit".to_string()),

            _ => Err("無効な入力です".to_string()),
        }
    }
}
