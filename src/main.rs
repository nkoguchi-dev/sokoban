mod presentation;

fn main() {
    let cli = presentation::cli::CLIAdapter::new();
    cli.run();
}
