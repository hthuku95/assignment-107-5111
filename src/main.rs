use clap::{Arg, Command};
use std::process;

mod note;
mod storage;
mod cli;
mod error;

use crate::cli::{create_note, list_notes, view_note, edit_note, delete_note, search_notes};
use crate::error::NoteError;

fn main() {
    let matches = Command::new("notes")
        .version("1.0.0")
        .author("Note App")
        .about("A simple note-taking application")
        .subcommand(
            Command::new("create")
                .about("Create a new note")
                .arg(
                    Arg::new("title")
                        .short('t')
                        .long("title")
                        .value_name("TITLE")
                        .help("Title of the note")
                        .required(true)
                )
                .arg(
                    Arg::new("content")
                        .short('c')
                        .long("content")
                        .value_name("CONTENT")
                        .help("Content of the note")
                        .required(false)
                )
                .arg(
                    Arg::new("tags")
                        .long("tags")
                        .value_name("TAGS")
                        .help("Comma-separated tags")
                        .required(false)
                )
        )
        .subcommand(
            Command::new("list")
                .about("List all notes")
                .arg(
                    Arg::new("tag")
                        .short('t')
                        .long("tag")
                        .value_name("TAG")
                        .help("Filter by tag")
                        .required(false)
                )
        )
        .subcommand(
            Command::new("view")
                .about("View a specific note")
                .arg(
                    Arg::new("id")
                        .help("Note ID to view")
                        .required(true)
                        .index(1)
                )
        )
        .subcommand(
            Command::new("edit")
                .about("Edit an existing note")
                .arg(
                    Arg::new("id")
                        .help("Note ID to edit")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("title")
                        .short('t')
                        .long("title")
                        .value_name("TITLE")
                        .help("New title for the note")
                        .required(false)
                )
                .arg(
                    Arg::new("content")
                        .short('c')
                        .long("content")
                        .value_name("CONTENT")
                        .help("New content for the note")
                        .required(false)
                )
                .arg(
                    Arg::new("tags")
                        .long("tags")
                        .value_name("TAGS")
                        .help("New comma-separated tags")
                        .required(false)
                )
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a note")
                .arg(
                    Arg::new("id")
                        .help("Note ID to delete")
                        .required(true)
                        .index(1)
                )
        )
        .subcommand(
            Command::new("search")
                .about("Search notes by content or title")
                .arg(
                    Arg::new("query")
                        .help("Search query")
                        .required(true)
                        .index(1)
                )
        )
        .get_matches();

    let result = match matches.subcommand() {
        Some(("create", sub_matches)) => {
            let title = sub_matches.get_one::<String>("title").unwrap();
            let content = sub_matches.get_one::<String>("content").unwrap_or(&String::new());
            let tags = sub_matches.get_one::<String>("tags")
                .map(|s| s.split(',').map(|tag| tag.trim().to