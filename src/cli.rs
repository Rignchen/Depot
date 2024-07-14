use crate::package_manager::PackageManager;
use clap::Parser;

structstruck::strike! {
    /// Structure of the command line arguments
    ///
    /// Usage: depot [OPTION] <COMMAND>
    ///
    /// Options:
    ///  -p, --pm, --package-manager <package-manager>   Manually select the package manager
    ///
    /// Commands:
    ///  install i    Install a package
    ///  remove  r    Remove a package
    ///  search  s    Search for a package
    ///  update  u    Update the package
    #[strikethrough[derive(Parser, Debug)]]
    pub struct Args {
        #[clap(short, long, alias = "pm", value_enum)]
        pub package_manager: Option<PackageManager>,
        #[clap(subcommand)]
        pub cmd: pub enum Command {
            #[clap(alias = "i")]
            Install(
                struct Install {
                    pub package: String,
                    #[clap(short, long)]
                    pub yes: bool,
                }
            ),
            #[clap(alias = "r")]
            Remove(
                struct Remove {
                    pub package: String,
                    #[clap(short, long)]
                    pub yes: bool,
                }
            ),
            #[clap(alias = "s")]
            Search(
                struct Search {
                    pub query: String,
                }
            ),
            #[clap(alias = "u")]
            Update(
                struct Update {
                    pub package: Option<String>,
                }
            ),
        }
    }
}
