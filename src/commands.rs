use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};
use serde_json;
use crate::note::{Note, NoteMetadata};
use crate::storage::Storage;
use crate::error::NoteError;

pub struct CommandHandler {
    storage: Storage,
}

impl CommandHandler {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    pub fn create_note(&mut self, title: String, content: String, tags: Vec<String>) -> Result<String, NoteError> {
        let note = Note::new(title, content, tags);
        let id = note.id.clone();
        
        self.storage.save_note(&note)?;
        
        println!("Note created successfully with ID: {}", id);
        Ok(id)
    }

    pub fn list_notes(&self, limit: Option<usize>) -> Result<Vec<NoteMetadata>, NoteError> {
        let mut notes = self.storage.list_notes()?;
        
        // Sort by creation date, newest first
        notes.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        if let Some(limit) = limit {
            notes.truncate(limit);
        }
        
        if notes.is_empty() {
            println!("No notes found.");
        } else {
            println!("Found {} note(s):", notes.len());
            for (index, note) in notes.iter().enumerate() {
                println!("{}. [{}] {} ({})", 
                    index + 1,
                    note.id[..8].to_string(),
                    note.title,
                    note.created_at.format("%Y-%m-%d %H:%M")
                );
                if !note.tags.is_empty() {
                    println!("   Tags: {}", note.tags.join(", "));
                }
            }
        }
        
        Ok(notes)
    }

    pub fn view_note(&self, id: &str) -> Result<Note, NoteError> {
        let note = self.storage.load_note(id)?;
        
        println!("Title: {}", note.title);
        println!("ID: {}", note.id);
        println!("Created: {}", note.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("Updated: {}", note.updated_at.format("%Y-%m-%d %H:%M:%S UTC"));
        
        if !note.tags.is_empty() {
            println!("Tags: {}", note.tags.join(", "));
        }
        
        println!("\nContent:");
        println!("{}", "-".repeat(50));
        println!("{}", note.content);
        println!("{}", "-".repeat(50));
        
        Ok(note)
    }

    pub fn update_note(&mut self, id: &str, title: Option<String>, content: Option<String>, tags: Option<Vec<String>>) -> Result<(), NoteError> {
        let mut note = self.storage.load_note(id)?;
        
        let mut updated = false;
        
        if let Some(new_title) = title {
            if new_title != note.title {
                note.title = new_title;
                updated = true;
            }
        }
        
        if let Some(new_content) = content {
            if new_content != note.content {
                note.content = new_content;
                updated = true;
            }
        }
        
        if let Some(new_tags) = tags {
            if new_tags != note.tags {
                note.tags = new_tags;
                updated = true;
            }
        }
        
        if updated {
            note.updated_at = Utc::now();
            self.storage.save_note(&note)?;
            println!("Note updated successfully.");
        } else {
            println!("No changes detected.");
        }
        
        Ok(())
    }

    pub fn delete_note(&mut self, id: &str