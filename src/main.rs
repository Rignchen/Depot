use depot::{
    cli::{parse_args, Command},
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
    let args = parse_args();
    let package_manager = unwrap_depot_error(get_package_manager(args.package_manager));
    match args.cmd {
        Command::Install(i) => unwrap_depot_error(package_manager.install(&i)),
        Command::Remove(r) => unwrap_depot_error(package_manager.remove(&r)),
        Command::Search(s) => unwrap_depot_error(package_manager.search(&s)),
        Command::Update(u) => unwrap_depot_error(package_manager.update(&u)),
    };
}
