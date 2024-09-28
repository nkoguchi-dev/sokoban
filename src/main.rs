mod application;
mod presentation;

use application::usecase::GameUseCase;
use presentation::cli::CLIAdapter;

fn main() {
    let usecase = GameUseCase::new();
    let mut cli = CLIAdapter::new(usecase);
    cli.run();
}
