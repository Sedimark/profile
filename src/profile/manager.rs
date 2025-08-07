use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Profile {
    pub alternate_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
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

    pub fn save(&self, profile: Profile) -> std::io::Result<()> {
        let mut stored_profile = self.profile.write().unwrap();
        *stored_profile = Some(profile);
        self.save_to_file(stored_profile.as_ref().unwrap())
    }

    pub fn get(&self) -> Option<Profile> {
        let profile = self.profile.read().unwrap();
        profile.clone()
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_manager() -> (ProfileManager, NamedTempFile) {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_str().unwrap().to_string();
        (ProfileManager::new(&path), file)
    }

    fn sample_profile() -> Profile {
        Profile {
            alternate_name: "testuser".to_string(),
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            company_name: Some("TestCo".to_string()),
            website: Some("https://test.com".to_string()),
            image_url: Some("https://test.com/avatar.png".to_string()),
        }
    }

    #[test]
    fn test_create_and_get_profile() {
        let (manager, _file) = create_test_manager();

        // Initially no profile
        assert!(manager.get().is_none());

        // Create a profile
        let profile = sample_profile();
        manager.save(profile.clone()).unwrap();

        // Verify we can retrieve it
        let retrieved = manager.get().unwrap();
        assert_eq!(retrieved, profile);
    }

    #[test]
    fn test_update_profile() {
        let (manager, _file) = create_test_manager();
        let profile = sample_profile();

        manager.save(profile.clone()).unwrap();

        // Update the profile
        let updated_profile = Profile {
            alternate_name: "testuser".to_string(),
            first_name: Some("Updated".to_string()),
            last_name: Some("New".to_string()),
            company_name: None,
            website: None,
            image_url: None,
        };
        manager.save(updated_profile.clone()).unwrap();

        let retrieved = manager.get().unwrap();
        assert_eq!(retrieved, updated_profile);
    }

    #[test]
    fn test_delete_profile() {
        let (manager, _file) = create_test_manager();
        let profile = sample_profile();

        manager.save(profile).unwrap();
        assert!(manager.get().is_some());

        manager.delete().unwrap();
        assert!(manager.get().is_none());
    }

    #[test]
    fn test_persistence() {
        let (manager, file) = create_test_manager();
        let profile = sample_profile();

        // Create and save
        manager.save(profile.clone()).unwrap();

        // Create new manager with same file
        let path = file.path().to_str().unwrap().to_string();
        let manager2 = ProfileManager::new(&path);

        // Should load the saved profile
        let retrieved = manager2.get().unwrap();
        assert_eq!(retrieved.first_name, profile.first_name);
    }
}
