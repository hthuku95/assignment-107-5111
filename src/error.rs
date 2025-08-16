use std::fmt;
use std::error::Error;
use std::io;

#[derive(Debug)]
pub enum NoteError {
    IoError(io::Error),
    SerializationError(String),
    ValidationError(String),
    NotFound(String),
    InvalidInput(String),
    DatabaseError(String),
}

impl fmt::Display for NoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NoteError::IoError(err) => write!(f, "IO error: {}", err),
            NoteError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            NoteError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            NoteError::NotFound(msg) => write!(f, "Not found: {}", msg),
            NoteError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            NoteError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl Error for NoteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NoteError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for NoteError {
    fn from(err: io::Error) -> Self {
        NoteError::IoError(err)
    }
}

impl From<serde_json::Error> for NoteError {
    fn from(err: serde_json::Error) -> Self {
        NoteError::SerializationError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, NoteError>;

pub fn validate_note_title(title: &str) -> Result<()> {
    if title.trim().is_empty() {
        return Err(NoteError::ValidationError("Title cannot be empty".to_string()));
    }
    
    if title.len() > 100 {
        return Err(NoteError::ValidationError("Title cannot exceed 100 characters".to_string()));
    }
    
    Ok(())
}

pub fn validate_note_content(content: &str) -> Result<()> {
    if content.len() > 10000 {
        return Err(NoteError::ValidationError("Content cannot exceed 10,000 characters".to_string()));
    }
    
    Ok(())
}

pub fn validate_tag(tag: &str) -> Result<()> {
    if tag.trim().is_empty() {
        return Err(NoteError::ValidationError("Tag cannot be empty".to_string()));
    }
    
    if tag.len() > 50 {
        return Err(NoteError::ValidationError("Tag cannot exceed 50 characters".to_string()));
    }
    
    if tag.contains(' ') {
        return Err(NoteError::ValidationError("Tags cannot contain spaces".to_string()));
    }
    
    Ok(())
}