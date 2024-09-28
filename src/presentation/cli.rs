use crate::application::usecase::{GameUseCase, InputCommands};
use std::io::{stdin, stdout};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct CLIAdapter {}

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
            // ユーザーにゲームの状態を表示
            self.display_game_state();
            let input = match c.unwrap() {
                termion::event::Key::Char('w') => InputCommands::UP,
                termion::event::Key::Char('s') => InputCommands::DOWN,
                termion::event::Key::Char('d') => InputCommands::LEFT,
                termion::event::Key::Char('a') => InputCommands::RIGHT,
                termion::event::Key::Char('q') | termion::event::Key::Ctrl('c') => {
                    InputCommands::QUIT
                }

                _ => {
                    println!("{}無効なキーが押されました", termion::clear::CurrentLine);
                    continue;
                }
            };
            match game_usecase.move_character(input) {
                Ok(str) => println!("キーが押されました。{}", str),
                Err(str) => {
                    println!("終了します。 {}", str);
                    break;
                }
            }
        }
    }

    fn display_init(&self) {
        print!("{}{}", termion::cursor::Goto(1, 1), termion::clear::All);
    }

    fn display_game_state(&self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    }
}
