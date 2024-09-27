use crate::application::usecase::GameUseCase;

pub struct CLIAdapter;

impl CLIAdapter {
    pub fn new() -> Self {
        CLIAdapter
    }

    pub fn run(&self) {
        let mut game_usecase = GameUseCase::new();
        loop {
            // ユーザーにゲームの状態を表示
            self.display_game_state();

            // ユーザーから入力を受け取る
            let input = self.get_user_input();
            match game_usecase.move_character(input.as_str()) {
                Err(str) => {
                    println!("終了します。 [{}]", str);
                    break;
                }
                _ => {}
            }
        }
    }

    fn display_game_state(&self) {
        // 未実装
    }

    // CLIでユーザーからの入力を取得する処理
    fn get_user_input(&self) -> String {
        println!("次の動きを入力してください (w/a/s/d/q): ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("入力に失敗しました");
        input.trim().to_string()
    }
}
