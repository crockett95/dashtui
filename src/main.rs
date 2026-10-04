use clap::{Parser, Subcommand};
use std::{path::PathBuf, process};

use dashtui::docset::find_docsets;
use dashtui::json::DiscoveryView;

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
}

fn fail(msg: &str, exit_code: i32) {
    eprintln!("{}", msg);
    process::exit(exit_code);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::List {
            root_dir,
            include_broken,
        } => match find_docsets(&root_dir) {
            Ok(docsets) => {
                let docsets = docsets
                    .into_iter()
                    .filter(|d| include_broken || d.meta.is_some())
                    .collect();
                if cli.json {
                    match serde_json::to_string(&DiscoveryView { docsets }) {
                        Ok(json) => {
                            println!("{}", json)
                        }
                        Err(error) => fail(&format!("Serialization failed: {}", error), 1),
                    }
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
            Err(e) => fail(&format!("an error occurred: {}", e), 1),
        },
    }
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
}
