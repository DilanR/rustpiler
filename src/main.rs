use rnr::cli::Cli;

fn main() {
    let cli = Cli::parse();
    if let Err(err) = cli.execute() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
