use crate::{
    cli::{Install, Remove, Search, Update},
    error::{DepotError, DepotResult},
    os::OperatingSystem,
};
use std::env;

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
    pub fn install(&self, instruction: &Install) -> String {
        match self {
            PackageManager::Pacman => format!("pacman -S {}", instruction.package),
            PackageManager::Yay => format!("yay -S {}", instruction.package),
            PackageManager::Apk => format!("apk add {}", instruction.package),
            PackageManager::AptGet => format!("apt-get install {}", instruction.package),
            PackageManager::Apt => format!("apt install {}", instruction.package),
            PackageManager::Pkg => format!("pkg install {}", instruction.package),
            PackageManager::Dnf => format!("dnf install {}", instruction.package),
        }
    }

    /// Remove a package using the package manager.
    pub fn remove(&self, instruction: &Remove) -> String {
        match self {
            PackageManager::Pacman => format!("pacman -R {}", instruction.package),
            PackageManager::Yay => format!("yay -R {}", instruction.package),
            PackageManager::Apk => format!("apk del {}", instruction.package),
            PackageManager::AptGet => format!("apt-get remove {}", instruction.package),
            PackageManager::Apt => format!("apt remove {}", instruction.package),
            PackageManager::Pkg => format!("pkg delete {}", instruction.package),
            PackageManager::Dnf => format!("dnf remove {}", instruction.package),
        }
    }

    /// Search for a package using the package manager.
    pub fn search(&self, instruction: &Search) -> String {
        match self {
            PackageManager::Pacman => format!("pacman -Ss {}", instruction.package),
            PackageManager::Yay => format!("yay -Ss {}", instruction.package),
            PackageManager::Apk => format!("apk search {}", instruction.package),
            PackageManager::AptGet => format!("apt-cache search {}", instruction.package),
            PackageManager::Apt => format!("apt search {}", instruction.package),
            PackageManager::Pkg => format!("pkg search {}", instruction.package),
            PackageManager::Dnf => format!("dnf search {}", instruction.package),
        }
    }

    /// Update one or all package using the package manager.
    pub fn update(&self, instruction: &Update) -> String {
        match self {
            PackageManager::Pacman => match &instruction.package {
                Some(package) => format!("pacman -S {}", package),
                None => "pacman -Syu".to_string(),
            },
            PackageManager::Yay => match &instruction.package {
                Some(package) => format!("yay -S {}", package),
                None => "yay -Syu".to_string(),
            },
            PackageManager::Apk => match &instruction.package {
                Some(package) => format!("apk add {}", package),
                None => "apk update && apk upgrade".to_string(),
            },
            PackageManager::AptGet => match &instruction.package {
                Some(package) => format!("apt-get install {}", package),
                None => "apt-get update && apt-get upgrade".to_string(),
            },
            PackageManager::Apt => match &instruction.package {
                Some(package) => format!("apt install {}", package),
                None => "apt update && apt upgrade".to_string(),
            },
            PackageManager::Pkg => match &instruction.package {
                Some(package) => format!("pkg install {}", package),
                None => "pkg upgrade".to_string(),
            },
            PackageManager::Dnf => match &instruction.package {
                Some(package) => format!("dnf install {}", package),
                None => "dnf upgrade".to_string(),
            },
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
