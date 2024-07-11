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
struct Args {}

/// Main function of the program
/// Parse the command line arguments
/// Look for the package manager in
///  - the command line arguments
///  - the environment variables
///  - get the os name and deduce it from there
fn main() {
    let args = Args::parse();
    println!("Hello, world!");
}
