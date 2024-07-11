use depot::{unwrap_depot_error, OperatingSystem};

/// Main function of the program
/// Parse the command line arguments
/// Look for the package manager in
///  - the command line arguments
///  - the environment variables
///  - get the os name and deduce it from there
fn main() {
    let os = unwrap_depot_error(OperatingSystem::current());
    println!("OS: {:?}", os);
}
