use clap::Parser;
use depot::package_manager::PackageManager;

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
    pub struct Args {
        #[clap(short, long, alias = "pm", value_enum)]
        pub package_manager: Option<PackageManager>,
        #[clap(subcommand)]
        pub cmd: pub enum Command {
            Install(
                struct Install {
                    pub package: String,
                }
            ),
            Remove(
                struct Remove {
                    pub package: String,
                }
            ),
            Search(
                struct Search {
                    pub query: String,
                }
            ),
            Update(
                struct Update {
                    pub package: Option<String>,
                }
            ),
        }
    }
}
