use clap::Parser;
use depot::{
    cli::{Args, Command},
    error::unwrap_depot_error,
    package_manager::get_package_manager,
};

/// Main function of the program
/// Parse the command line arguments
/// Look for the package manager in
///  - the command line arguments
///  - the environment variables
///  - get the os name and deduce it from there
fn main() {
    let args = Args::parse();
    let package_manager = unwrap_depot_error(get_package_manager(args.package_manager));
    println!("Package manager: {:?}", package_manager);
    println!(
        "{}",
        match args.cmd {
            Command::Install(i) => format!("Install package: {}", i.package),
            Command::Remove(r) => format!("Remove package: {}", r.package),
            Command::Search(s) => format!("Search for package: {}", s.query),
            Command::Update(u) => format!(
                "Update package list: {}",
                u.package.unwrap_or("all".to_string())
            ),
        }
    );
}
