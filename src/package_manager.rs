use crate::{
    cli::{Install, Remove, Search, Update},
    error::{DepotError, DepotResult, PackageManagerError as error},
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

/// Run a command with the package manager.
/// This macro is used to avoid code duplication between all the package manager and their methods.
macro_rules! run_command {
    ($name:expr, $arg1:expr, $packages:expr, $arg2: expr, $instruction:expr) => {{
        let mut command = Command::new($name);
        command.arg($arg1);
        if $instruction.yes {
            command.arg($arg2);
        }
        command.args($packages);
        command
    }};
    ($name:expr, $arg1:expr, $packages:expr) => {{
        let mut command = Command::new($name);
        command.arg($arg1);
        command.arg($packages);
        command
    }};
    ($name:expr, $all:expr, $arg1:expr, $packages:expr) => {{
        let mut command = Command::new($name);
        match $packages {
            Some(package) => command.arg($arg1).args(package),
            None => command.arg($all),
        };
        command
    }};
}

impl PackageManager {
    /// Install a package using the package manager.
    pub fn install(&self, instruction: &Install) -> DepotResult<()> {
        let result = match self {
            PackageManager::Pacman => run_command!(
                "pacman",
                "-S",
                &instruction.package,
                "--noconfirm",
                instruction
            ),
            PackageManager::Yay => run_command!(
                "yay",
                "-S",
                &instruction.package,
                "--noconfirm",
                instruction
            ),
            PackageManager::Apk => run_command!(
                "apk",
                "add",
                &instruction.package,
                "--no-cache",
                instruction
            ),
            PackageManager::AptGet => run_command!(
                "apt-get",
                "install",
                &instruction.package,
                "-y",
                instruction
            ),
            PackageManager::Apt => {
                run_command!("apt", "install", &instruction.package, "-y", instruction)
            }
            PackageManager::Pkg => {
                run_command!("pkg", "install", &instruction.package, "-y", instruction)
            }
            PackageManager::Dnf => {
                run_command!("dnf", "install", &instruction.package, "-y", instruction)
            }
        }
        .status();
        if result.is_ok() && result.unwrap().success() {
            Ok(())
        } else {
            Err(DepotError::PackageManagerError(
                error::InstallFailed(instruction.package.clone()),
                self.clone(),
            ))
        }
    }

    /// Remove a package using the package manager.
    pub fn remove(&self, instruction: &Remove) -> DepotResult<()> {
        let result = match self {
            PackageManager::Pacman => run_command!(
                "pacman",
                "-R",
                &instruction.package,
                "--noconfirm",
                instruction
            ),
            PackageManager::Yay => run_command!(
                "yay",
                "-R",
                &instruction.package,
                "--noconfirm",
                instruction
            ),
            PackageManager::Apk => run_command!(
                "apk",
                "del",
                &instruction.package,
                "--no-cache",
                instruction
            ),
            PackageManager::AptGet => {
                run_command!("apt-get", "remove", &instruction.package, "-y", instruction)
            }
            PackageManager::Apt => {
                run_command!("apt", "remove", &instruction.package, "-y", instruction)
            }
            PackageManager::Pkg => {
                run_command!("pkg", "remove", &instruction.package, "-y", instruction)
            }
            PackageManager::Dnf => {
                run_command!("dnf", "remove", &instruction.package, "-y", instruction)
            }
        }
        .status();
        if result.is_ok() && result.unwrap().success() {
            Ok(())
        } else {
            Err(DepotError::PackageManagerError(
                error::RemoveFailed(instruction.package.clone()),
                self.clone(),
            ))
        }
    }

    /// Search for a package using the package manager.
    pub fn search(&self, instruction: &Search) -> DepotResult<()> {
        let result = match self {
            PackageManager::Pacman => {
                run_command!("pacman", "-Ss", &instruction.package)
            }
            PackageManager::Yay => run_command!("yay", "-Ss", &instruction.package),
            PackageManager::Apk => run_command!("apk", "search", &instruction.package),
            PackageManager::AptGet => {
                run_command!("apt-cache", "search", &instruction.package)
            }
            PackageManager::Apt => run_command!("apt", "search", &instruction.package),
            PackageManager::Pkg => run_command!("pkg", "search", &instruction.package),
            PackageManager::Dnf => run_command!("dnf", "search", &instruction.package),
        }
        .status();
        if result.is_ok() && result.unwrap().success() {
            Ok(())
        } else {
            Err(DepotError::PackageManagerError(
                error::SearchFailed(instruction.package.clone()),
                self.clone(),
            ))
        }
    }

    /// Update one or all package using the package manager.
    pub fn update(&self, instruction: &Update) -> DepotResult<()> {
        let result = match self {
            PackageManager::Pacman => run_command!("pacman", "-Syu", "-S", &instruction.package),
            PackageManager::Yay => run_command!("yay", "-Syu", "-S", &instruction.package),
            PackageManager::Apk => run_command!("apk", "upgrade", "upgrade", &instruction.package),
            PackageManager::AptGet => {
                run_command!("apt-get", "upgrade", "upgrade", &instruction.package)
            }
            PackageManager::Apt => run_command!("apt", "upgrade", "upgrade", &instruction.package),
            PackageManager::Pkg => run_command!("pkg", "upgrade", "upgrade", &instruction.package),
            PackageManager::Dnf => run_command!("dnf", "upgrade", "upgrade", &instruction.package),
        }
        .status();
        if result.is_ok() && result.unwrap().success() {
            Ok(())
        } else {
            Err(DepotError::PackageManagerError(
                error::UpdateFailed(instruction.package.clone()),
                self.clone(),
            ))
        }
    }

    /// Test if the package manager is installed.
    /// If it is not installed, return an error.
    /// ```
    /// use depot::package_manager::PackageManager;
    /// assert!(PackageManager::Pacman.ensure_pm_installed().is_ok());
    /// ```
    /// ```should_panic
    /// use depot::package_manager::PackageManager;
    /// PackageManager::Dnf.ensure_pm_installed().unwrap();
    /// ```
    pub fn ensure_pm_installed(&self) -> DepotResult<Self> {
        let temp = Command::new("which").arg(format!("{:?}", self).to_lowercase()).output();
        if temp.is_ok() && temp.unwrap().status.success() {
            Ok(self.clone())
        } else {
            Err(DepotError::PackageManagerError(
                error::PackageManagerNotInstalled,
                self.clone(),
            ))
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
