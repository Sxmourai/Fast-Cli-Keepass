use std::{
    fs::File,
    io::Write,
    path::PathBuf,
};

fn get_config_path() -> Option<PathBuf> {
    let dirs = directories::ProjectDirs::from("dev", "Sxmourai", "Fast Cli Keepass").unwrap();
    Some(dirs.config_dir().join("config.toml"))
}

pub fn get_config() -> Option<File> {
    std::fs::File::open(get_config_path().unwrap()).ok()
}
#[derive(Debug)]
pub enum ConfigCreateError {
    AlreadyCreated,
    CantCreate,
}

/// Creates the file if it doesn't exist
pub fn ensure_created() -> Option<()> {
    std::fs::DirBuilder::new().create(directories::ProjectDirs::from("dev", "Sxmourai", "Fast Cli Keepass").unwrap().config_dir()).unwrap();
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(get_config_path().unwrap())
        .unwrap();
    file.write(&[]).unwrap(); // Create empty file if doesn't exist
    file.flush().unwrap();
    Some(())
}
