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
        let result = match self {
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
        .status();
        if result.is_ok() && result.unwrap().success() {
            Ok(())
        } else {
            Err(DepotError::PackageManagerError)
        }
    }

    /// Remove a package using the package manager.
    pub fn remove(&self, instruction: &Remove) -> DepotResult<()> {
        let result = match self {
            PackageManager::Pacman => {
                let mut command = Command::new("pacman");
                command.arg("-R");
                if instruction.yes {
                    command.arg("--noconfirm");
                }
                command.args(&instruction.package);
                command
            }
            PackageManager::Yay => {
                let mut command = Command::new("yay");
                command.arg("-R");
                if instruction.yes {
                    command.arg("--noconfirm");
                }
                command.args(&instruction.package);
                command
            }
            PackageManager::Apk => {
                let mut command = Command::new("apk");
                command.arg("del");
                if instruction.yes {
                    command.arg("--no-cache");
                }
                command.args(&instruction.package);
                command
            }
            PackageManager::AptGet => {
                let mut command = Command::new("apt-get");
                command.arg("remove");
                if instruction.yes {
                    command.arg("-y");
                }
                command.args(&instruction.package);
                command
            }
            PackageManager::Apt => {
                let mut command = Command::new("apt");
                command.arg("remove");
                if instruction.yes {
                    command.arg("-y");
                }
                command.args(&instruction.package);
                command
            }
            PackageManager::Pkg => {
                let mut command = Command::new("pkg");
                command.arg("remove");
                if instruction.yes {
                    command.arg("-y");
                }
                command.args(&instruction.package);
                command
            }
            PackageManager::Dnf => {
                let mut command = Command::new("dnf");
                command.arg("remove");
                if instruction.yes {
                    command.arg("-y");
                }
                command.args(&instruction.package);
                command
            }
        }
        .status();
        if result.is_ok() && result.unwrap().success() {
            Ok(())
        } else {
            Err(DepotError::PackageManagerError)
        }
    }

    /// Search for a package using the package manager.
    pub fn search(&self, instruction: &Search) -> DepotResult<()> {
        let result = match self {
            PackageManager::Pacman => {
                let mut command = Command::new("pacman");
                command.arg("-Ss");
                command.arg(&instruction.package);
                command
            }
            PackageManager::Yay => {
                let mut command = Command::new("yay");
                command.arg("-Ss");
                command.arg(&instruction.package);
                command
            }
            PackageManager::Apk => {
                let mut command = Command::new("apk");
                command.arg("search");
                command.arg(&instruction.package);
                command
            }
            PackageManager::AptGet => {
                let mut command = Command::new("apt-cache");
                command.arg("search");
                command.arg(&instruction.package);
                command
            }
            PackageManager::Apt => {
                let mut command = Command::new("apt");
                command.arg("search");
                command.arg(&instruction.package);
                command
            }
            PackageManager::Pkg => {
                let mut command = Command::new("pkg");
                command.arg("search");
                command.arg(&instruction.package);
                command
            }
            PackageManager::Dnf => {
                let mut command = Command::new("dnf");
                command.arg("search");
                command.arg(&instruction.package);
                command
            }
        }
        .status();
        if result.is_ok() && result.unwrap().success() {
            Ok(())
        } else {
            Err(DepotError::PackageManagerError)
        }
    }

    /// Update one or all package using the package manager.
    pub fn update(&self, instruction: &Update) -> DepotResult<()> {
        let result = match self {
            PackageManager::Pacman => {
                let mut command = Command::new("pacman");
                match &instruction.package {
                    Some(package) => command.arg("-S").args(package),
                    None => command.arg("-Syu"),
                };
                command
            }
            PackageManager::Yay => {
                let mut command = Command::new("yay");
                match &instruction.package {
                    Some(package) => command.arg("-S").args(package),
                    None => command.arg("-Syu"),
                };
                command
            }
            PackageManager::Apk => {
                let mut command = Command::new("apk");
                match &instruction.package {
                    Some(package) => command.arg("upgrade").args(package),
                    None => command.arg("upgrade"),
                };
                command
            }
            PackageManager::AptGet => {
                let mut command = Command::new("apt-get");
                match &instruction.package {
                    Some(package) => command.arg("upgrade").args(package),
                    None => command.arg("upgrade"),
                };
                command
            }
            PackageManager::Apt => {
                let mut command = Command::new("apt");
                match &instruction.package {
                    Some(package) => command.arg("upgrade").args(package),
                    None => command.arg("upgrade"),
                };
                command
            }
            PackageManager::Pkg => {
                let mut command = Command::new("pkg");
                match &instruction.package {
                    Some(package) => command.arg("upgrade").args(package),
                    None => command.arg("upgrade"),
                };
                command
            }
            PackageManager::Dnf => {
                let mut command = Command::new("dnf");
                match &instruction.package {
                    Some(package) => command.arg("upgrade").args(package),
                    None => command.arg("upgrade"),
                };
                command
            }
        }
        .status();
        if result.is_ok() && result.unwrap().success() {
            Ok(())
        } else {
            Err(DepotError::PackageManagerError)
        }
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
