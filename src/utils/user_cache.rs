// use dirs::cache_dir;
use crate::commands::mkwrs::Record;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize)]
pub struct UserData {
    pub seen_tracks: HashSet<Record>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct UserCache {
    users: HashMap<String, UserData>,
}

impl UserCache {
    fn get_cache_path(file_name: &str) -> io::Result<PathBuf> {
        if let Ok(dir) = std::env::var("FERRISBOT_STATE_DIR") {
            let mut path = PathBuf::from(dir);
            fs::create_dir_all(&path)?;
            path.push(file_name);
            return Ok(path);
        }

        // fallback to default
        let mut path = dirs::cache_dir().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Could not find cache directory")
        })?;
        path.push("ferrisbot");
        fs::create_dir_all(&path)?;
        path.push(file_name);
        Ok(path)
    }

    pub fn load(file_name: &str) -> io::Result<Self> {
        let path = Self::get_cache_path(file_name)?;
        if !path.exists() {
            return Ok(Self::default());
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    pub fn save(&self, file_name: &str) -> io::Result<()> {
        let path = Self::get_cache_path(file_name)?;
        let data = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn get_seen_tracks(&self, user: &str) -> HashSet<Record> {
        self.users
            .get(user)
            .map(|data| data.seen_tracks.clone())
            .unwrap_or_default()
    }

    pub fn update_seen_tracks(&mut self, user: &str, entries: HashSet<Record>) {
        self.users.entry(user.to_string()).or_default().seen_tracks = entries;
    }
}
