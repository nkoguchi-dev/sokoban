mod application;
mod domain;
mod presentation;

use application::usecase::GameUseCase;
//use domain::game::GameDomainService;
use presentation::cli::CLIAdapter;

fn main() {
    //let game_domain_service = GameDomainService;
    let usecase = GameUseCase::new(/*game_domain_service*/);
    let mut cli = CLIAdapter::new(usecase);
    cli.run();
}
