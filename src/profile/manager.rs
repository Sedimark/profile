use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub first_name: String,
    pub last_name: String,
    pub company_name: String,
    pub website: String,
    pub image_url: String,
}

pub struct ProfileManager {
    profile: RwLock<Option<Profile>>,
    file_path: String,
}

impl ProfileManager {
    pub fn new(file_path: &str) -> Self {
        let profile = if Path::new(file_path).exists() {
            let contents = fs::read_to_string(file_path).unwrap_or_default();
            serde_json::from_str(&contents).ok()
        } else {
            None
        };

        ProfileManager {
            profile: RwLock::new(profile),
            file_path: file_path.to_string(),
        }
    }

    pub fn create(&self, profile: Profile) -> std::io::Result<()> {
        let mut stored_profile = self.profile.write().unwrap();
        *stored_profile = Some(profile.clone());
        self.save_to_file(&profile)
    }

    pub fn get(&self) -> Option<Profile> {
        let profile = self.profile.read().unwrap();
        profile.clone()
    }

    pub fn update(&self, profile: Profile) -> std::io::Result<()> {
        let mut stored_profile = self.profile.write().unwrap();
        *stored_profile = Some(profile.clone());
        self.save_to_file(&profile)
    }

    pub fn delete(&self) -> std::io::Result<()> {
        let mut profile = self.profile.write().unwrap();
        *profile = None;
        self.delete_file()
    }

    fn save_to_file(&self, profile: &Profile) -> std::io::Result<()> {
        let serialized = serde_json::to_string_pretty(profile)?;
        fs::write(&self.file_path, serialized)
    }

    fn delete_file(&self) -> std::io::Result<()> {
        if Path::new(&self.file_path).exists() {
            fs::remove_file(&self.file_path)
        } else {
            Ok(())
        }
    }
}
