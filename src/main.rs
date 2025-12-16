use rnr::cli::Cli;

fn main() {
    let cli = Cli::parse();
    let _ = cli.execute();
}
