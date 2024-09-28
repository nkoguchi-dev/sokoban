use crate::application::usecase::{GameUseCase, InputCommands};
use std::io::{stdin, stdout};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct CLIAdapter {}

#[derive(Debug)]
enum InputError {
    Quit,
    Interrupt,
    OtherKey,
}

impl CLIAdapter {
    pub fn new() -> Self {
        CLIAdapter {}
    }

    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();
        let mut game_usecase = GameUseCase::new();

        self.display_init();
        for c in stdin.keys() {
            self.display_game_state();
            let command = match self.get_user_input(c.unwrap()) {
                Ok(command) => command,
                Err(input_error) => match input_error {
                    InputError::Quit | InputError::Interrupt => {
                        println!("終了します。 {:?}", input_error);
                        break;
                    }
                    InputError::OtherKey => continue,
                },
            };
            match game_usecase.move_character(command) {
                Ok(str) => println!("キーが押されました。{}", str),
                Err(str) => {
                    println!("ゲーム処理でのエラー。 {}", str);
                    break;
                }
            }
        }
        game_usecase.quit_game();
    }

    fn display_init(&self) {
        print!("{}{}", termion::cursor::Goto(1, 1), termion::clear::All);
    }

    fn display_game_state(&self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    }

    fn get_user_input(&self, key: termion::event::Key) -> Result<InputCommands, InputError> {
        match key {
            termion::event::Key::Char('w') => Ok(InputCommands::UP),
            termion::event::Key::Char('s') => Ok(InputCommands::DOWN),
            termion::event::Key::Char('d') => Ok(InputCommands::LEFT),
            termion::event::Key::Char('a') => Ok(InputCommands::RIGHT),
            termion::event::Key::Char('q') => Err(InputError::Quit),
            termion::event::Key::Ctrl('c') => Err(InputError::Interrupt),
            _ => Err(InputError::OtherKey),
        }
    }
}
