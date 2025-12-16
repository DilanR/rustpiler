use rnr::cli::Cli;

fn main() {
    let cli = Cli::parse();
    cli.execute();
}
