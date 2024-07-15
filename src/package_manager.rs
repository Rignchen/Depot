use crate::{
    cli::{Install, Remove, Search, Update},
    error::{DepotError, DepotResult},
    os::OperatingSystem,
};
use std::env;
use std::process::Command;

/// List of all supported operating systems.
#[derive(clap::ValueEnum, Debug, Clone, PartialEq)]
pub enum PackageManager {
    Pacman,
    Yay,
    Apk,
    AptGet,
    Apt,
    Pkg,
    Dnf,
}

impl From<&OperatingSystem> for PackageManager {
    /// Get the default package manager for the operating system.
    fn from(os: &OperatingSystem) -> PackageManager {
        match os {
            OperatingSystem::Arch => PackageManager::Pacman,
            OperatingSystem::Alpine => PackageManager::Apk,
            OperatingSystem::Debian => PackageManager::AptGet,
            OperatingSystem::Ubuntu => PackageManager::AptGet,
            OperatingSystem::Fedora => PackageManager::Dnf,
        }
    }
}

impl PackageManager {
    /// Install a package using the package manager.
    pub fn install(&self, instruction: &Install) -> DepotResult<()> {
        match self {
            PackageManager::Pacman => {
                let mut command = Command::new("pacman");
                command.arg("-S");
                if instruction.yes {
                    command.arg("--noconfirm");
                }
                command.args(&instruction.package);
                command
            }
            PackageManager::Yay => {
                let mut command = Command::new("yay");
                command.arg("-S");
                if instruction.yes {
                    command.arg("--noconfirm");
                }
                command.args(&instruction.package);
                command
            }
            PackageManager::Apk => {
                let mut command = Command::new("apk");
                command.arg("add");
                if instruction.yes {
                    command.arg("--no-cache");
                }
                command.args(&instruction.package);
                command
            }
            PackageManager::AptGet => {
                let mut command = Command::new("apt-get");
                command.arg("install");
                if instruction.yes {
                    command.arg("-y");
                }
                command.args(&instruction.package);
                command
            }
            PackageManager::Apt => {
                let mut command = Command::new("apt");
                command.arg("install");
                if instruction.yes {
                    command.arg("-y");
                }
                command.args(&instruction.package);
                command
            }
            PackageManager::Pkg => {
                let mut command = Command::new("pkg");
                command.arg("install");
                if instruction.yes {
                    command.arg("-y");
                }
                command.args(&instruction.package);
                command
            }
            PackageManager::Dnf => {
                let mut command = Command::new("dnf");
                command.arg("install");
                if instruction.yes {
                    command.arg("-y");
                }
                command.args(&instruction.package);
                command
            }
        }
        .status()
        .unwrap();
        Ok(())
    }

    /// Remove a package using the package manager.
    pub fn remove(&self, instruction: &Remove) -> DepotResult<()> {
        match self {
            PackageManager::Pacman => println!("pacman -R {}", instruction.package.join(" ")),
            PackageManager::Yay => println!("yay -R {}", instruction.package.join(" ")),
            PackageManager::Apk => println!("apk del {}", instruction.package.join(" ")),
            PackageManager::AptGet => println!("apt-get remove {}", instruction.package.join(" ")),
            PackageManager::Apt => println!("apt remove {}", instruction.package.join(" ")),
            PackageManager::Pkg => println!("pkg delete {}", instruction.package.join(" ")),
            PackageManager::Dnf => println!("dnf remove {}", instruction.package.join(" ")),
        };
        Ok(())
    }

    /// Search for a package using the package manager.
    pub fn search(&self, instruction: &Search) -> DepotResult<()> {
        match self {
            PackageManager::Pacman => println!("pacman -Ss {}", instruction.package),
            PackageManager::Yay => println!("yay -Ss {}", instruction.package),
            PackageManager::Apk => println!("apk search {}", instruction.package),
            PackageManager::AptGet => println!("apt-cache search {}", instruction.package),
            PackageManager::Apt => println!("apt search {}", instruction.package),
            PackageManager::Pkg => println!("pkg search {}", instruction.package),
            PackageManager::Dnf => println!("dnf search {}", instruction.package),
        };
        Ok(())
    }

    /// Update one or all package using the package manager.
    pub fn update(&self, instruction: &Update) -> DepotResult<()> {
        match self {
            PackageManager::Pacman => match &instruction.package {
                Some(package) => println!("pacman -S {}", package.join(" ")),
                None => println!("pacman -Syu"),
            },
            PackageManager::Yay => match &instruction.package {
                Some(package) => println!("yay -S {}", package.join(" ")),
                None => println!("yay -Syu"),
            },
            PackageManager::Apk => match &instruction.package {
                Some(package) => println!("apk add {}", package.join(" ")),
                None => println!("apk update && apk upgrade"),
            },
            PackageManager::AptGet => match &instruction.package {
                Some(package) => println!("apt-get install {}", package.join(" ")),
                None => println!("apt-get update && apt-get upgrade"),
            },
            PackageManager::Apt => match &instruction.package {
                Some(package) => println!("apt install {}", package.join(" ")),
                None => println!("apt update && apt upgrade"),
            },
            PackageManager::Pkg => match &instruction.package {
                Some(package) => println!("pkg install {}", package.join(" ")),
                None => println!("pkg upgrade"),
            },
            PackageManager::Dnf => match &instruction.package {
                Some(package) => println!("dnf install {}", package.join(" ")),
                None => println!("dnf upgrade"),
            },
        };
        Ok(())
    }
}

/// Get the package manager to use.
/// If the expected one is None, look for it in the environment variables.
/// If it is not in the environment variables, guess it from the current operating system.
///
/// ```
/// use depot::{package_manager::{get_package_manager, PackageManager}, os::OperatingSystem};
/// assert_eq!(get_package_manager(Some(PackageManager::Pacman)).unwrap(), PackageManager::Pacman);
/// std::env::set_var("DEPOT_PM", "yay");
/// assert_eq!(get_package_manager(None).unwrap(), PackageManager::Yay);
/// std::env::remove_var("DEPOT_PM");
/// assert_eq!(get_package_manager(None).unwrap(),PackageManager::from(&OperatingSystem::current().unwrap()));
/// ```
/// ```should_panic
/// std::env::set_var("DEPOT_PM", "unknown");
/// depot::package_manager::get_package_manager(None).unwrap();
/// ```
pub fn get_package_manager(expected: Option<PackageManager>) -> DepotResult<PackageManager> {
    match expected {
        Some(manager) => Ok(manager),
        None => match env::var("DEPOT_PM") {
            Ok(manager) => match manager.as_str() {
                "pacman" => Ok(PackageManager::Pacman),
                "yay" => Ok(PackageManager::Yay),
                "apk" => Ok(PackageManager::Apk),
                "apt-get" => Ok(PackageManager::AptGet),
                "apt" => Ok(PackageManager::Apt),
                "pkg" => Ok(PackageManager::Pkg),
                "dnf" => Ok(PackageManager::Dnf),
                _ => Err(DepotError::UnknownPackageManager),
            },
            Err(_) => {
                let os = OperatingSystem::current()?;
                Ok(PackageManager::from(&os))
            }
        },
    }
}
