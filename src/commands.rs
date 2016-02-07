pub mod node {
    use setup;
    use logger;
    use archive_reader;
    use symlink;
    use std;
    use downloader;
    use ls;
    use system_node;
    use std::io::ErrorKind;

    pub fn install(version: String) {
        let home_directory = match setup::prepare() {
            Ok(directory) => {
                directory
            },
            Err(err) => {
                logger::stderr("Failed to initialize home directory");
                logger::stderr(format!("{:?}", err));
                std::process::exit(1)
            }
        };

        if setup::has_version(&version) {
            logger::stderr(format!("Version {} is already installed", version));
            std::process::exit(1)
        }

        let downloader = downloader::Downloader::new();
        let archive_path = match downloader.download_source(&version, &home_directory) {
            Ok(path)  => path,
            Err(err)    => {
                logger::stderr(format!("Download failed:\n{}", err));
                std::process::exit(1)
            }
        };
        let destination_path = setup::avm_directory();
        logger::stdout(format!("Unzipping to {}", destination_path));

        match setup::create_version_directory(&version) {
            Ok(_) => { },
            Err(err) => {
                logger::stderr(format!("Failed to create directory for version\n{}", err));
                std::process::exit(1)
            }
        };

        match archive_reader::decompress(&archive_path, destination_path, &version) {
            Ok(_) => { },
            Err(err) => logger::stderr(format!("Error occured\n{}", err))
        };

        match archive_reader::remove_archive_file(&archive_path) {
            Ok(_) => { },
            Err(err) => logger::stderr(format!("Error occured while removing archive file\n{}", err))
        };

        logger::success(format!("Successfully installed version {}", version));
        logger::success(format!("Run avm use {} to use it", version));
    }

    pub fn remove_symlink() {
        match symlink::remove_symlink() {
            Err(err) => {
                if err.kind() != std::io::ErrorKind::NotFound {
                    logger::stderr("Failed to remove symlink");
                    logger::stderr(format!("{:?}", err));
                    std::process::exit(1)
                }
            },
            _ => { }
        };
    }

    pub fn use_version(version: String) {
        if setup::has_version(&version) {
            remove_symlink();
            match symlink::symlink_to_version(&version) {
                Ok(_) => logger::success(format!("Now using node v{}", version)),
                Err(err) => {
                    logger::stderr("Failed to set symlink");
                    logger::stderr(format!("{:?}", err));
                    std::process::exit(1)
                }
            };
        }
        else if version == "system" {
            remove_symlink();
            match symlink::create_symlinks_to_system_binaries() {
                Ok(_)       => logger::stdout("using system node"),
                Err(err)    => {
                    if err.kind() == ErrorKind::NotFound {
                        logger::stderr(format!("It appears that there's no node.js preinstalled on this system"));
                        return;
                    } else {
                        logger::stderr(format!("{:?}", err));
                    }
                }
            }
        }
        else {
            logger::stderr(format!("Version {} not installed", version));
            std::process::exit(1)
        }
    }

    pub fn list_versions() {
        let current_version = match ls::current_version() {
            Some(v) => v,
            None => Default::default()
        };

        let system_version = match system_node::version() {
            Ok(v) => v,
            Err(_) => Default::default()
        };
        let mut installed_versions = ls::ls_versions();
        if system_version != Default::default() {
            installed_versions.push(system_version.clone());
        }

        logger::stdout(format!("Listing all installed versions:"));
        logger::stdout(format!("(=>): current version"));

        for installed_version in installed_versions {
            let system_suffix = if installed_version == system_version {
                String::from("(system)")
            } else {
                String::new()
            };

            if installed_version.path == current_version.path {
                logger::stdout(format!("=> {} {}", installed_version.name, system_suffix));
            }
            else if installed_version != Default::default() {
                logger::stdout(format!("- {} {}", installed_version.name, system_suffix));
            }
        }
    }

    pub fn uninstall(version: String) {
        if setup::has_version(&version) {

            if symlink::points_to_version(&version) {
                match symlink::remove_symlink() {
                    Err(err) => {
                        if err.kind() != std::io::ErrorKind::NotFound {
                            logger::stderr("Failed to remove symlink");
                            logger::stderr(format!("{:?}", err));
                            std::process::exit(1)
                        }
                    },
                    _ => { }
                }
            }

            match setup::remove_version(&version) {
                Ok(_) => logger::stdout(format!("Successfully removed version {}", version)),
                Err(err) => {
                    logger::stderr(format!("Failed to remove version {}", version));
                    logger::stderr(format!("{:?}", err));
                }
            }
        }
        else {
            logger::stderr(format!("Version {} is not installed", version));
            std::process::exit(1)
        }
    }
}
