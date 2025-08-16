use crate::note::{Note, NoteManager};
use crate::storage::{FileStorage, Storage};
use crate::search::SearchEngine;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[cfg(test)]
mod note_tests {
    use super::*;

    #[test]
    fn test_note_creation() {
        let note = Note::new("Test Title", "Test content");
        
        assert_eq!(note.title, "Test Title");
        assert_eq!(note.content, "Test content");
        assert!(!note.id.is_empty());
        assert!(note.created_at.timestamp() > 0);
        assert!(note.updated_at.timestamp() > 0);
    }

    #[test]
    fn test_note_update() {
        let mut note = Note::new("Original Title", "Original content");
        let original_created_at = note.created_at;
        
        std::thread::sleep(std::time::Duration::from_millis(10));
        note.update_content("Updated content");
        
        assert_eq!(note.content, "Updated content");
        assert_eq!(note.created_at, original_created_at);
        assert!(note.updated_at > original_created_at);
    }

    #[test]
    fn test_note_update_title() {
        let mut note = Note::new("Original Title", "Content");
        let original_created_at = note.created_at;
        
        std::thread::sleep(std::time::Duration::from_millis(10));
        note.update_title("New Title");
        
        assert_eq!(note.title, "New Title");
        assert_eq!(note.created_at, original_created_at);
        assert!(note.updated_at > original_created_at);
    }

    #[test]
    fn test_note_add_tag() {
        let mut note = Note::new("Title", "Content");
        
        note.add_tag("important");
        note.add_tag("work");
        
        assert!(note.tags.contains(&"important".to_string()));
        assert!(note.tags.contains(&"work".to_string()));
        assert_eq!(note.tags.len(), 2);
    }

    #[test]
    fn test_note_add_duplicate_tag() {
        let mut note = Note::new("Title", "Content");
        
        note.add_tag("important");
        note.add_tag("important");
        
        assert_eq!(note.tags.len(), 1);
    }

    #[test]
    fn test_note_remove_tag() {
        let mut note = Note::new("Title", "Content");
        
        note.add_tag("important");
        note.add_tag("work");
        note.remove_tag("important");
        
        assert!(!note.tags.contains(&"important".to_string()));
        assert!(note.tags.contains(&"work".to_string()));
        assert_eq!(note.tags.len(), 1);
    }
}

#[cfg(test)]
mod note_manager_tests {
    use super::*;

    fn create_temp_storage() -> (TempDir, FileStorage) {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileStorage::new(temp_dir.path().to_str().unwrap()).unwrap();
        (temp_dir, storage)
    }

    #[test]
    fn test_note_manager_creation() {
        let (_temp_dir, storage) = create_temp_storage();
        let manager = NoteManager::new(Box::new(storage));
        
        assert_eq!(manager.list_notes().len(), 0);
    }

    #[test]
    fn test_add_note() {
        let (_temp_dir, storage) = create_temp_storage();
        let mut manager = NoteManager::new(Box::new(storage));
        
        let note_id = manager.add_note("Test Title", "Test content").unwrap();
        
        assert!(!note_id.