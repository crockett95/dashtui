use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

use dashtui::docset::{Docset, find_docsets};
use dashtui::json::{DiscoveryView, SearchView, StatsView};

#[derive(Debug, Parser)]
/// dashtui: Dash docs for the terminal
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Return the result as JSON
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand, Debug, PartialEq, Eq)]
enum Command {
    /// List all docsets found in a directory
    ///
    /// Prints every `*.docset` directory found under `<dir>`,
    /// one path per line, using `dashtui::docset::find_docsets`.
    List {
        /// Directory to search for docsets
        root_dir: PathBuf,
        /// Include docsets with broken metadata
        #[arg(long)]
        include_broken: bool,
    },
    /// Show the counts for each entry type found in the docset
    ///
    /// Prints every entry type found in the docset along with the total
    /// count of entries matching that type
    Stats {
        /// Docset directory
        docset_dir: PathBuf,
    },
    /// Search a docset for entries matching the query
    ///
    /// Returns the top entries from the docset matching query
    Search {
        /// Docset directory
        docset_dir: PathBuf,
        /// Search query
        query: String,
        /// Maximum number of entries to return
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::List {
            root_dir,
            include_broken,
        } => list(&root_dir, include_broken, cli.json)?,
        Command::Stats { docset_dir } => stats(docset_dir, cli.json)?,
        Command::Search {
            docset_dir,
            query,
            limit,
        } => search(docset_dir, &query, limit, cli.json)?,
    }
    Ok(())
}

/// `list` subcommand implementation. See [`Command::List`]
fn list(root_dir: &Path, include_broken: bool, json: bool) -> Result<()> {
    let docsets = find_docsets(root_dir)?
        .into_iter()
        .filter(|d| include_broken || d.meta().is_some())
        .collect();
    if json {
        let json = serde_json::to_string(&DiscoveryView { docsets })?;
        println!("{}", json);
    } else {
        for docset in &docsets {
            let name = if let Some(meta) = docset.meta() {
                &meta.name
            } else {
                ""
            };
            println!("{}\t{}", name, docset.path().display());
        }
    }
    Ok(())
}

/// `stats` subcommand implementation. See [`Command::Stats`]
fn stats(docset_dir: PathBuf, json: bool) -> Result<()> {
    let docset = Docset::load(docset_dir);
    let counts = docset.load_stats()?;
    if json {
        let json = serde_json::to_string(&StatsView { docset, counts })?;
        println!("{}", json);
    } else {
        for (key, count) in &counts {
            println!("{:<12}\t{:>5}", key, count);
        }
    }
    Ok(())
}

/// `search` subcommand implementation. See [`Command::Search`]
fn search(docset_dir: PathBuf, query: &str, limit: usize, json: bool) -> Result<()> {
    let docset = Docset::load(docset_dir);
    let mut results = docset.search(query)?;
    results.truncate(limit);
    if json {
        let json = serde_json::to_string(&SearchView {
            docset: &docset,
            query,
            results,
        })?;
        println!("{}", json);
    } else {
        println!("Results for '{}' in {}", query, docset.path().display());
        println!("{:<50}\t{:<12}\tPath (with Anchor)", "Name", "Type");
        for entry in results {
            let path = if let Some(anchor) = entry.anchor() {
                format!("{}#{}", entry.path(), anchor)
            } else {
                entry.path().to_string()
            };
            println!("{:<50}\t{:<12}\t{}", entry.name(), entry.kind(), path);
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn list_subcommand_parsing() {
        assert_eq!(
            Cli::try_parse_from(["dashtui", "list", "some/dir"])
                .expect("Test failed")
                .command,
            Command::List {
                root_dir: PathBuf::from("some/dir"),
                include_broken: false
            }
        )
    }

    #[test]
    fn list_fails_if_no_dir() {
        let result = Cli::try_parse_from(["dashtui", "list"]);
        assert!(result.is_err());
    }

    #[test]
    fn stats_subcommand_parsing() {
        assert_eq!(
            Cli::try_parse_from(["dashtui", "stats", "some/dir"])
                .expect("Test failed")
                .command,
            Command::Stats {
                docset_dir: PathBuf::from("some/dir"),
            }
        )
    }

    #[test]
    fn stats_fails_if_no_dir() {
        let result = Cli::try_parse_from(["dashtui", "stats"]);
        assert!(result.is_err());
    }

    #[test]
    fn search_subcommand_parsing() {
        assert_eq!(
            Cli::try_parse_from(["dashtui", "search", "some/dir", "query"])
                .expect("Test failed")
                .command,
            Command::Search {
                docset_dir: PathBuf::from("some/dir"),
                query: String::from("query"),
                limit: 20
            }
        )
    }

    #[test]
    fn search_subcommand_parsing_takes_limit() {
        assert_eq!(
            Cli::try_parse_from(["dashtui", "search", "--limit", "40", "some/dir", "query"])
                .expect("Test failed")
                .command,
            Command::Search {
                docset_dir: PathBuf::from("some/dir"),
                query: String::from("query"),
                limit: 40
            }
        )
    }

    #[test]
    fn search_fails_if_no_query() {
        let result = Cli::try_parse_from(["dashtui", "search", "some/dir"]);
        assert!(result.is_err());
    }
}
