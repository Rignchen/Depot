use crate::package_manager::PackageManager;
use clap::Parser;

structstruck::strike! {
    /// Every one has once encountered the problem of learning a new package manager when changing their operating systems.
    /// Depot solves this problem by providing a unified interface to package managers.
    /// When you ask Depot to install a package it will ask your package manager to install it for you.
    /// That way you keep all advantages of your package manager while using a unified interface.
    #[strikethrough[derive(Parser, Debug)]]
    #[clap(verbatim_doc_comment)]
    pub struct Args {
        #[clap(short, long, alias = "pm", value_enum)]
        pub package_manager: Option<PackageManager>,
        #[clap(subcommand)]
        pub cmd: pub enum Command {
            /// Install a package
            #[clap(alias = "i")]
            Install(
                struct Install {
                    pub package: Vec<String>,
                    /// Install the package without asking for confirmation
                    #[clap(short, long)]
                    pub yes: bool,
                }
            ),
            /// Remove a package
            #[clap(alias = "r")]
            Remove(
                struct Remove {
                    pub package: Vec<String>,
                    /// Install the package without asking for confirmation
                    #[clap(short, long)]
                    pub yes: bool,
                }
            ),
            /// Search for a package
            #[clap(alias = "s")]
            Search(
                struct Search {
                    pub package: String,
                }
            ),
            /// Update one or all packages
            #[clap(alias = "u")]
            Update(
                struct Update {
                    pub package: Option<Vec<String>>,
                    /// Install the package without asking for confirmation
                    #[clap(short, long)]
                    pub yes: bool,
                }
            ),
        }
    }
}

pub fn parse_args() -> Args {
    let mut args = Args::parse();
    // due to a bug in clap, we need to ensure that at lesat one argument package is provided
    match &mut args.cmd {
        Command::Install(install) => {
            if install.package.is_empty() {
                eprintln!("Error: No package provided");
                std::process::exit(1);
            }
        }
        Command::Remove(remove) => {
            if remove.package.is_empty() {
                eprintln!("Error: No package provided");
                std::process::exit(1);
            }
        }
        _ => {}
    }
    args
}
