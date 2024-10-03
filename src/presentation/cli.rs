use crate::application::usecase::{GameUseCase, InputCommands};
use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct CLIAdapter {
    usecase: GameUseCase,
}

#[derive(Debug)]
enum InputError {
    Quit,
    Interrupt,
    OtherKey,
}

impl CLIAdapter {
    pub fn new(usecase: GameUseCase) -> Self {
        CLIAdapter { usecase }
    }

    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();

        self.usecase.start_game(8, 8);
        self.display_game_state();
        for c in stdin.keys() {
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
            match self.usecase.move_character(command) {
                Ok(str) => println!("キーが押されました。{}", str),
                Err(str) => {
                    println!("ゲーム処理でのエラー。 {}", str);
                    break;
                }
            }
            self.display_game_state();
        }
        self.usecase.quit_game();
    }

    fn display_game_state(&self) {
        println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        let query_output = self.usecase.get_display_model();
        for y in 0..query_output.map.len() {
            let v = &query_output.map[y];
            for x in 0..v.len() {
                let pos_x: u16 = x.try_into().unwrap();
                let pos_y: u16 = y.try_into().unwrap();
                print!("{}{}", termion::cursor::Goto(pos_x + 1, pos_y + 1), v[x]);
            }
        }
        let under_map_position_y: u16 = query_output.map.len().try_into().unwrap();
        print!("{}", termion::cursor::Goto(0, under_map_position_y + 1));
        stdout().flush().unwrap();
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
