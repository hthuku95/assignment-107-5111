use clap::{Arg, ArgMatches, Command, value_parser};
use std::path::PathBuf;

pub struct CliArgs {
    pub command: CliCommand,
}

#[derive(Debug, Clone)]
pub enum CliCommand {
    Create {
        title: String,
        content: Option<String>,
        tags: Vec<String>,
    },
    List {
        tag: Option<String>,
        limit: Option<usize>,
    },
    Show {
        id: u64,
    },
    Edit {
        id: u64,
        title: Option<String>,
        content: Option<String>,
        tags: Option<Vec<String>>,
    },
    Delete {
        id: u64,
        force: bool,
    },
    Search {
        query: String,
        in_content: bool,
    },
    Tag {
        id: u64,
        tags: Vec<String>,
        remove: bool,
    },
    Export {
        format: ExportFormat,
        output: Option<PathBuf>,
        tag: Option<String>,
    },
    Import {
        file: PathBuf,
        format: ImportFormat,
    },
}

#[derive(Debug, Clone)]
pub enum ExportFormat {
    Json,
    Markdown,
    Txt,
}

#[derive(Debug, Clone)]
pub enum ImportFormat {
    Json,
    Markdown,
}

impl CliArgs {
    pub fn parse() -> Result<Self, String> {
        let matches = build_cli().get_matches();
        let command = parse_command(&matches)?;
        Ok(CliArgs { command })
    }

    pub fn parse_from<I, T>(args: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let matches = build_cli().try_get_matches_from(args)
            .map_err(|e| e.to_string())?;
        let command = parse_command(&matches)?;
        Ok(CliArgs { command })
    }
}

fn build_cli() -> Command {
    Command::new("notes")
        .version("1.0.0")
        .author("Notes App")
        .about("A powerful command-line note-taking application")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("create")
                .about("Create a new note")
                .alias("new")
                .alias("add")
                .arg(
                    Arg::new("title")
                        .help("Title of the note")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("content")
                        .help("Content of the note")
                        .short('c')
                        .long("content")
                        .value_name("TEXT")
                )
                .arg(
                    Arg::new("tags")
                        .help("Tags for the note")
                        .short('t')
                        .long("tags")
                        .value_name("TAG")
                        .action(clap::ArgAction::Append)
                )
        )
        .subcommand(
            Command::new("list")
                .about("List all notes")
                .alias("ls")
                .arg(
                    Arg::new("tag")
                        .help("Filter by tag")
                        .short('t')
                        .long("tag")
                        .value_name("TAG")
                )
                .arg(
                    Arg::new("limit")
                        .help("Limit number of results")
                        .short('l')
                        .long("limit")
                        .value_name("NUMBER")
                        .value_parser(value_parser!(usize))
                )
        )
        .subcommand(
            Command::new("show")
                .about("Show a specific note")
                .alias("view")
                .arg(
                    Arg