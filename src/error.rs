use crate::package_manager::PackageManager;

structstruck::strike! {
    /// List of errors that the program can return.
    #[strikethrough[derive(Debug)]]
    pub enum DepotError {
        UnknownOperatingSystem,
        UnknownPackageManager,
        PackageManagerError(
            enum PackageManagerError {
                InstallFailed(Vec<String>),
                RemoveFailed(Vec<String>),
                SearchFailed(String),
                UpdateFailed(Option<Vec<String>>),
            },
            PackageManager
        ),
    }
}

/// Result type which wither take a type T or a DepotError.
pub type DepotResult<T> = std::result::Result<T, DepotError>;
pub fn unwrap_depot_error<T>(result: DepotResult<T>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => {
            eprintln!(
                "{}",
                match error {
                    DepotError::UnknownOperatingSystem =>
                        "Unable to determine your current operating system.".to_string(),
                    DepotError::UnknownPackageManager =>
                        "The package manager is unknown or not supported.".to_string(),
                    DepotError::PackageManagerError(pme, pm) => match pme {
                        PackageManagerError::InstallFailed(package) => format!(
                            "Failed to install package: {} using {:?}",
                            package.join(", "),
                            pm
                        ),
                        PackageManagerError::RemoveFailed(package) => format!(
                            "Failed to remove package: {} using {:?}",
                            package.join(", "),
                            pm
                        ),
                        PackageManagerError::SearchFailed(package) =>
                            format!("Failed to search for package: {} using {:?}", package, pm),
                        PackageManagerError::UpdateFailed(package) => match package {
                            Some(package) => format!(
                                "Failed to update package: {} using {:?}",
                                package.join(", "),
                                pm
                            ),
                            None => "Failed to update all packages.".to_string(),
                        },
                    },
                }
            );
            std::process::exit(1);
        }
    }
}
