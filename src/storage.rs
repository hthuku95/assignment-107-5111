use std::fs;
use std::path::Path;
use std::io::{self, Write};
use serde_json;
use crate::note::Note;

pub struct FileStorage {
    storage_dir: String,
}

impl FileStorage {
    pub fn new(storage_dir: &str) -> io::Result<Self> {
        let path = Path::new(storage_dir);
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        
        Ok(FileStorage {
            storage_dir: storage_dir.to_string(),
        })
    }

    pub fn save_note(&self, note: &Note) -> io::Result<()> {
        let filename = format!("{}.json", note.id);
        let file_path = Path::new(&self.storage_dir).join(filename);
        
        let json_data = serde_json::to_string_pretty(note)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        let mut file = fs::File::create(file_path)?;
        file.write_all(json_data.as_bytes())?;
        file.flush()?;
        
        Ok(())
    }

    pub fn load_note(&self, id: &str) -> io::Result<Note> {
        let filename = format!("{}.json", id);
        let file_path = Path::new(&self.storage_dir).join(filename);
        
        if !file_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Note with id '{}' not found", id),
            ));
        }
        
        let json_data = fs::read_to_string(file_path)?;
        let note: Note = serde_json::from_str(&json_data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        Ok(note)
    }

    pub fn delete_note(&self, id: &str) -> io::Result<()> {
        let filename = format!("{}.json", id);
        let file_path = Path::new(&self.storage_dir).join(filename);
        
        if !file_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Note with id '{}' not found", id),
            ));
        }
        
        fs::remove_file(file_path)?;
        Ok(())
    }

    pub fn list_notes(&self) -> io::Result<Vec<Note>> {
        let mut notes = Vec::new();
        
        let entries = fs::read_dir(&self.storage_dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                match fs::read_to_string(&path) {
                    Ok(json_data) => {
                        match serde_json::from_str::<Note>(&json_data) {
                            Ok(note) => notes.push(note),
                            Err(e) => {
                                eprintln!("Warning: Failed to parse note file {:?}: {}", path, e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to read note file {:?}: {}", path, e);
                    }
                }
            }
        }
        
        notes.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(notes)
    }

    pub fn search_notes(&self, query: &str) -> io::Result<Vec<Note>> {
        let all_notes = self.list_notes()?;
        let query_lower = query.to_lowercase();
        
        let filtered_notes: Vec<Note> = all_