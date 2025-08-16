use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_archived: bool,
    pub metadata: HashMap<String, String>,
}

impl Note {
    pub fn new(title: String, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            is_archived: false,
            metadata: HashMap::new(),
        }
    }

    pub fn with_tags(title: String, content: String, tags: Vec<String>) -> Self {
        let mut note = Self::new(title, content);
        note.tags = tags;
        note
    }

    pub fn update_content(&mut self, new_content: String) {
        self.content = new_content;
        self.updated_at = Utc::now();
    }

    pub fn update_title(&mut self, new_title: String) {
        self.title = new_title;
        self.updated_at = Utc::now();
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_tag(&mut self, tag: &str) -> bool {
        if let Some(pos) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(pos);
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }

    pub fn archive(&mut self) {
        self.is_archived = true;
        self.updated_at = Utc::now();
    }

    pub fn unarchive(&mut self) {
        self.is_archived = false;
        self.updated_at = Utc::now();
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.updated_at = Utc::now();
    }

    pub fn remove_metadata(&mut self, key: &str) -> Option<String> {
        let result = self.metadata.remove(key);
        if result.is_some() {
            self.updated_at = Utc::now();
        }
        result
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    pub fn matches_search(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        self.title.to_lowercase().contains(&query_lower)
            || self.content.to_lowercase().contains(&query_lower)
            || self.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
    }

    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }

    pub fn character_count(&self) -> usize {
        self.content.chars().count()
    }

    pub fn is_empty(&self) -> bool {
        self.title.trim().is_empty() && self.content.trim().is_empty()
    }

    pub fn get_preview(&self, max_chars: usize) -> String {
        if self.content.len() <= max_chars {
            self.content.clone()
        } else {
            let mut preview = self.content.chars().take(max_chars).collect: