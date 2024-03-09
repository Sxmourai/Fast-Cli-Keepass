use std::{fs::File, io::Write, path::PathBuf};

fn get_config_path() -> PathBuf {
    let dirs = directories::ProjectDirs::from("dev", "Sxmourai", "Fast Cli Keepass").expect("Failed getting config dir");
    dirs.config_dir().join("config.toml")
}

pub fn get_config() -> Option<File> {
    std::fs::File::open(get_config_path()).ok()
}
#[derive(Debug)]
pub enum ConfigCreateError {
    AlreadyCreated,
    CantCreate,
}

/// Creates the file if it doesn't exist
pub fn ensure_created() -> Option<()> {
    match std::fs::DirBuilder::new()
        .create(
            directories::ProjectDirs::from("dev", "Sxmourai", "Fast Cli Keepass")
                .unwrap()
                .config_dir(),
        ) {
            Ok(_) => {},
            Err(err) => match err.kind() {
                std::io::ErrorKind::AlreadyExists => {},
                _ => panic!("Error creating config directory"),
            },
        }
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(get_config_path())
        .unwrap();
    file.write(&[]).unwrap(); // Create empty file if doesn't exist
    file.flush().unwrap();
    Some(())
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AppConfig {
    pub db_path: PathBuf,
}
impl AppConfig {
    /// Creates a new config, and writes to *CONFIG*/config.toml
    pub fn new(db_path: PathBuf) -> Self {
        let _self = Self {
            db_path: db_path.clone(),
        };
        let raw_config = toml::to_string_pretty(&_self).unwrap();
        let mut create_ops = std::fs::OpenOptions::new();
        create_ops.create(true).write(true);
        match create_ops
            .open(get_config_path()) // TODO Ask user for config dir ? But then how do we know on next restart ?
            .and_then(|mut f| {
                f.write_all(raw_config.as_bytes())
            })
        {
            Ok(_path) => {println!("Written config to {}", db_path.display())}
            Err(err) => {
                panic!("Can't create/write to db: {db_path:?} because {err:?}")
            }
        }
        _self
    }
    pub fn read() -> Self {
        let raw = std::fs::read_to_string(get_config_path()).expect("Failed getting/reading config file");
        toml::from_str(&raw).expect("Failed parsing toml config")
    }
}
