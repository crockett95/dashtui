use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use dashtui::docset::{Docset, find_docsets};
use dashtui::json::{DiscoveryView, StatsView};

#[derive(Debug, Parser)]
/// dashtui: Dash docs for the terminal
struct Cli {
    #[command(subcommand)]
    command: Command,

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
    Stats {
        /// Docset directory
        docset_dir: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::List {
            root_dir,
            include_broken,
        } => {
            let docsets = find_docsets(&root_dir)?
                .into_iter()
                .filter(|d| include_broken || d.meta.is_some())
                .collect();
            if cli.json {
                let json = serde_json::to_string(&DiscoveryView { docsets })?;
                println!("{}", json);
            } else {
                for docset in docsets.iter() {
                    let name = if let Some(meta) = &docset.meta {
                        &meta.name
                    } else {
                        ""
                    };
                    println!("{}\t{}", name, docset.path.to_string_lossy());
                }
            }
        }
        Command::Stats { docset_dir } => {
            let docset = Docset::load(docset_dir);
            let counts = docset.load_stats()?;
            if cli.json {
                let json = serde_json::to_string(&StatsView { docset, counts })?;
                println!("{}", json);
            } else {
                for (key, count) in counts.iter() {
                    println!("{:<12}{:>5}", key, count);
                }
            }
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
}
