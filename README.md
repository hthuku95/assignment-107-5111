# Rust Note Taking Application

A fast, efficient, and user-friendly note-taking application built with Rust. This command-line tool allows you to create, manage, and organize your notes with powerful search and categorization features.

## Features

- **Fast Performance**: Built with Rust for optimal speed and memory efficiency
- **Simple CLI Interface**: Easy-to-use command-line interface
- **Note Management**: Create, edit, delete, and list notes
- **Search Functionality**: Quick search through your notes
- **Categories/Tags**: Organize notes with tags and categories
- **Data Persistence**: Notes are saved locally in a structured format
- **Cross-Platform**: Works on Windows, macOS, and Linux

## Installation

### Prerequisites

- Rust 1.70.0 or higher
- Cargo (comes with Rust)

### From Source

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rust-notes-app.git
cd rust-notes-app
```

2. Build the application:
```bash
cargo build --release
```

3. Install globally (optional):
```bash
cargo install --path .
```

### Using Cargo

```bash
cargo install rust-notes-app
```

## Quick Start

1. Create your first note:
```bash
notes add "My First Note" --content "This is my first note content"
```

2. List all notes:
```bash
notes list
```

3. Search for notes:
```bash
notes search "first"
```

## Usage

### Basic Commands

#### Create a New Note
```bash
# Create a note with title and content
notes add "Meeting Notes" --content "Discussion points for today's meeting"

# Create a note with tags
notes add "Project Ideas" --content "Ideas for new projects" --tags "work,projects,ideas"

# Create a note interactively
notes add --interactive
```

#### List Notes
```bash
# List all notes
notes list

# List notes with specific tag
notes list --tag "work"

# List notes with limit
notes list --limit 10
```

#### View a Note
```bash
# View note by ID
notes show 1

# View note by title (partial match)
notes show "Meeting"
```

#### Edit a Note
```bash
# Edit note content
notes edit 1 --content "Updated content"

# Edit note title
notes edit 1 --title "Updated Title"

# Edit note tags
notes edit 1 --tags "updated,tags"

# Interactive edit
notes edit 1 --interactive
```

#### Search Notes
```bash
# Search in titles and content
notes search "meeting"

# Search with case sensitivity
notes search "Meeting" --case-sensitive

# Search in specific fields
notes search "project" --field title
```

#### Delete Notes
```bash
# Delete a specific note
notes delete 1

# Delete multiple notes
notes delete 1 2 3

# Delete all notes with confirmation
notes delete --all
```

### Advanced Features

#### Tags and Categories
```bash
# Add tags to existing note
notes tag add 1 "important" "urgent"

# Remove tags from note
notes tag remove 1 "urgent"

# List all tags
notes tags list

# Find notes by tag
notes find --tag "important"
```

#### Export and Import
```bash
# Export notes to JSON
notes export --format json --output notes_backup.json

# Export to markdown
notes export --format markdown --output notes.md

# Import from JSON
notes import notes_backup.json
```

#### Configuration
```bash
# Set default editor
notes config set editor "vim"

# Set notes directory
notes config set notes_dir "/path/to/notes"

# View current configuration
notes config show
```

## Configuration

The application stores configuration in `~/.config/rust-notes/config.toml`:

```toml
[general]
notes_dir = "~/.local/share/rust-notes"
default_editor = "nano"
date_format = "%Y-%m-%d %H:%M:%S"

[display]
max_title_length = 50
show_tags = true
show_dates = true

[search]
case_sensitive = false
fuzzy_search = true
```

## File Structure

```
~/.local/share/rust-notes/
├── notes/
│   ├── 001_my_first