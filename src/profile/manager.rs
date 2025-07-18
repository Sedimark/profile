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
    card: RwLock<Option<Profile>>,
    file_path: String,
}

impl ProfileManager {
    pub fn new(file_path: &str) -> Self {
        let card = if Path::new(file_path).exists() {
            let contents = fs::read_to_string(file_path).unwrap_or_default();
            serde_json::from_str(&contents).ok()
        } else {
            None
        };

        ProfileManager {
            card: RwLock::new(card),
            file_path: file_path.to_string(),
        }
    }

    pub fn create(&self, card: Profile) -> std::io::Result<()> {
        let mut stored_card = self.card.write().unwrap();
        *stored_card = Some(card.clone());
        self.save_to_file(&card)
    }

    pub fn get(&self) -> Option<Profile> {
        let card = self.card.read().unwrap();
        card.clone()
    }

    pub fn update(&self, card: Profile) -> std::io::Result<()> {
        let mut stored_card = self.card.write().unwrap();
        *stored_card = Some(card.clone());
        self.save_to_file(&card)
    }

    pub fn delete(&self) -> std::io::Result<()> {
        let mut card = self.card.write().unwrap();
        *card = None;
        self.delete_file()
    }

    fn save_to_file(&self, card: &Profile) -> std::io::Result<()> {
        let serialized = serde_json::to_string_pretty(card)?;
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
