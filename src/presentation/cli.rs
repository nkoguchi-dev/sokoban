pub struct CLIAdapter;

impl CLIAdapter {
    pub fn new() -> Self {
        CLIAdapter
    }

    pub fn run(&self) {
        loop {
            // ユーザーにゲームの状態を表示
            self.display_game_state();

            // ユーザーから入力を受け取る
            let input = self.get_user_input();
            match input.as_str() {
                "w" | "a" | "s" | "d" => println!("input: {}", input),
                "q" => break,
                _ => continue,
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
