use dirs;
use std::{fs, path::PathBuf};

pub struct SessionStorage;

impl SessionStorage {
    fn get_file() -> Option<PathBuf> {
        let mut home = dirs::home_dir()?;
        home.push(".project_euler_session");
        Some(home)
    }

    pub fn get() -> Option<String> {
        fs::read_to_string(SessionStorage::get_file()?).ok()
    }

    pub fn save(sess: String) -> Option<()> {
        fs::write(SessionStorage::get_file()?, sess.as_bytes()).ok()
    }

    pub fn destroy() -> Option<()> {
        fs::remove_file(SessionStorage::get_file()?).ok()
    }
}
