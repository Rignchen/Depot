use clap::Parser;
use depot::{unwrap_depot_error, PackageManager, get_package_manager};

structstruck::strike! {
    /// Structure of the command line arguments
    ///
    /// Usage: depot [OPTION] <COMMAND>
    ///
    /// Options:
    ///  -p, --pm, --package-manager <package-manager>   Manually select the package manager
    ///
    /// Commands:
    ///  install    Install a package
    ///  remove     Remove a package
    ///  search     Search for a package
    ///  update     Update the package
    #[strikethrough[derive(Parser, Debug)]]
    struct Args {
        #[clap(short, long, alias = "pm", value_enum)]
        package_manager: Option<PackageManager>,
        #[clap(subcommand)]
        cmd: enum Command {
            Install(
                struct Install {
                    package: String,
                }
            ),
            Remove(
                struct Remove {
                    package: String,
                }
            ),
            Search(
                struct Search {
                    query: String,
                }
            ),
            Update(
                struct Update {
                    package: Option<String>,
                }
            ),
        }
    }
}

/// Main function of the program
/// Parse the command line arguments
/// Look for the package manager in
///  - the command line arguments
///  - the environment variables
///  - get the os name and deduce it from there
fn main() {
    let args = Args::parse();
    let package_manager = get_package_manager(args.package_manager);
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
