use clap::Parser;
use depot::{
    cli::{Args, Command},
    error::{unwrap_depot_error, DepotResult},
    package_manager::get_package_manager,
};

/// Entry point of the program
/// Handle the error returned by the program
fn main() {
    unwrap_depot_error(run());
}

/// Main function of the program
/// Parse the command line arguments
/// Look for the package manager in
///  - the command line arguments
///  - the environment variables
///  - get the os name and deduce it from there
fn run() -> DepotResult<()> {
    let args = Args::parse();
    let package_manager = get_package_manager(args.package_manager)?.ensure_pm_installed()?;
    match args.cmd {
        Command::Install(i) => package_manager.install(&i)?,
        Command::Remove(r) => package_manager.remove(&r)?,
        Command::Search(s) => package_manager.search(&s)?,
        Command::Update(u) => package_manager.update(&u)?,
    };
    Ok(())
}
