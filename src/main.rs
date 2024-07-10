use depot::OperatingSystem;

/// Main function of the program
/// Parse the command line arguments
/// Look for the package manager in
///  - the command line arguments
///  - the environment variables
///  - get the os name and deduce it from there
fn main() {
    let os = OperatingSystem::current().unwrap();
    println!("OS: {:?}", os);
}
