//! # Note Taking Application
//! 
//! A simple and efficient note-taking application built with Rust.
//! This library provides core functionality for creating, managing, and organizing notes.

pub mod note;
pub mod storage;
pub mod search;
pub mod error;
pub mod config;

pub use note::{Note, NoteId, Priority, Tag};
pub use storage::{Storage, FileStorage};
pub use search::{SearchEngine, SearchResult};
pub use error::{NoteError, Result};
pub use config::Config;

use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Main application struct that orchestrates all note-taking operations
pub struct NoteApp {
    storage: Box<dyn Storage>,
    search_engine: SearchEngine,
    config: Config,
}

impl NoteApp {
    /// Creates a new note application instance
    pub fn new(storage: Box<dyn Storage>, config: Config) -> Result<Self> {
        let search_engine = SearchEngine::new();
        Ok(Self {
            storage,
            search_engine,
            config,
        })
    }

    /// Creates a new note application with file-based storage
    pub fn with_file_storage(storage_path: &str) -> Result<Self> {
        let config = Config::default();
        let storage = Box::new(FileStorage::new(storage_path)?);
        Self::new(storage, config)
    }

    /// Creates a new note
    pub fn create_note(&mut self, title: String, content: String) -> Result<NoteId> {
        let note = Note::new(title, content);
        let note_id = note.id();
        self.storage.save_note(&note)?;
        self.search_engine.index_note(&note);
        Ok(note_id)
    }

    /// Retrieves a note by its ID
    pub fn get_note(&self, id: &NoteId) -> Result<Option<Note>> {
        self.storage.load_note(id)
    }

    /// Updates an existing note
    pub fn update_note(&mut self, id: &NoteId, title: Option<String>, content: Option<String>) -> Result<()> {
        if let Some(mut note) = self.storage.load_note(id)? {
            if let Some(new_title) = title {
                note.set_title(new_title);
            }
            if let Some(new_content) = content {
                note.set_content(new_content);
            }
            note.update_modified_time();
            self.storage.save_note(&note)?;
            self.search_engine.update_note(&note);
        }
        Ok(())
    }

    /// Deletes a note
    pub fn delete_note(&mut self, id: &NoteId) -> Result<bool> {
        let deleted = self.storage.delete_note(id)?;
        if deleted {
            self.search_engine.remove_note(id);
        }
        Ok(deleted)
    }

    /// Lists all notes
    pub fn list_notes(&self) -> Result<Vec<Note>> {
        self.storage.list_notes()
    }

    /// Searches notes by query
    pub fn search_notes(&self, query: &str) -> Result<Vec<SearchResult>> {
        Ok(self.search_engine.search(query))
    }

    /// Adds a tag to a note
    pub fn add_tag(&mut self, note_id: &NoteId, tag: Tag) -> Result<()> {
        if let Some(mut note) = self.storage.load_note(note_id)? {
            note.add_tag(tag);
            note.update_modified_time();
            self.storage.save_note(&note)?;
            self.search_engine.update_note(&note);
        }
        Ok(())
    }

    /// Removes a tag from a note
    pub fn remove_tag(&mut self, note_id: &NoteId, tag: &Tag) -> Result<()> {
        if let Some(mut note) = self.storage.load_note(note_id)? {
            note.remove_tag(tag