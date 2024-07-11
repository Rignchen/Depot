use clap::Parser;

/// Structure of the command line arguments
///
/// Usage: depot [COMMAND]
///
/// Commands:
///  install    Install a package
///  remove     Remove a package
///  search     Search for a package
///  update     Update the package list
#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    cmd: Command,
}
#[derive(Parser)]
enum Command {
    Install(Install),
    Remove(Remove),
    Search(Search),
    Update(Update),
}
#[derive(Parser)]
struct Install {
    package: String,
}
#[derive(Parser)]
struct Remove {
    package: String,
}
#[derive(Parser)]
struct Search {
    query: String,
}
#[derive(Parser)]
struct Update {}

/// Main function of the program
/// Parse the command line arguments
/// Look for the package manager in
///  - the command line arguments
///  - the environment variables
///  - get the os name and deduce it from there
fn main() {
    let args = Args::parse();
    println!("{}", match args.cmd {
        Command::Install(i) => format!("Install package: {}", i.package),
        Command::Remove(r) => format!("Remove package: {}", r.package),
        Command::Search(s) => format!("Search for package: {}", s.query),
        Command::Update(_) => "Update the package list".to_string(),
    });
}
