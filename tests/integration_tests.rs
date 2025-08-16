use std::fs;
use std::path::Path;
use tempfile::TempDir;
use crate::note::Note;
use crate::note_manager::NoteManager;
use crate::storage::Storage;

#[test]
fn test_full_note_lifecycle() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage_path = temp_dir.path().join("notes");
    
    let mut storage = Storage::new(storage_path.clone());
    let mut manager = NoteManager::new(storage);
    
    // Create a new note
    let note_id = manager.create_note("Test Title".to_string(), "Test content".to_string())
        .expect("Failed to create note");
    
    // Verify note was created
    let note = manager.get_note(&note_id).expect("Failed to get note");
    assert_eq!(note.title(), "Test Title");
    assert_eq!(note.content(), "Test content");
    
    // Update the note
    manager.update_note(&note_id, Some("Updated Title".to_string()), Some("Updated content".to_string()))
        .expect("Failed to update note");
    
    // Verify update
    let updated_note = manager.get_note(&note_id).expect("Failed to get updated note");
    assert_eq!(updated_note.title(), "Updated Title");
    assert_eq!(updated_note.content(), "Updated content");
    
    // List all notes
    let notes = manager.list_notes().expect("Failed to list notes");
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].id(), &note_id);
    
    // Delete the note
    manager.delete_note(&note_id).expect("Failed to delete note");
    
    // Verify deletion
    assert!(manager.get_note(&note_id).is_err());
    let notes_after_delete = manager.list_notes().expect("Failed to list notes after delete");
    assert_eq!(notes_after_delete.len(), 0);
}

#[test]
fn test_multiple_notes_management() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage_path = temp_dir.path().join("notes");
    
    let mut storage = Storage::new(storage_path.clone());
    let mut manager = NoteManager::new(storage);
    
    // Create multiple notes
    let note1_id = manager.create_note("Note 1".to_string(), "Content 1".to_string())
        .expect("Failed to create note 1");
    let note2_id = manager.create_note("Note 2".to_string(), "Content 2".to_string())
        .expect("Failed to create note 2");
    let note3_id = manager.create_note("Note 3".to_string(), "Content 3".to_string())
        .expect("Failed to create note 3");
    
    // Verify all notes exist
    let notes = manager.list_notes().expect("Failed to list notes");
    assert_eq!(notes.len(), 3);
    
    // Verify each note has correct content
    let note1 = manager.get_note(&note1_id).expect("Failed to get note 1");
    let note2 = manager.get_note(&note2_id).expect("Failed to get note 2");
    let note3 = manager.get_note(&note3_id).expect("Failed to get note 3");
    
    assert_eq!(note1.title(), "Note 1");
    assert_eq!(note2.title(), "Note 2");
    assert_eq!(note3.title(), "Note 3");
    
    // Delete middle note
    manager.delete_note(&note2_id).expect("Failed to delete note 2");
    
    // Verify only 2 notes remain
    let remaining_notes = manager.list_notes().expect("Failed to list remaining notes");
    assert_eq!(remaining_notes.len(), 2);
    
    // Verify correct notes remain
    assert!(manager.get_note(&note1_id).is