use std::{env::args, path::Path, process};

use dashtui::docset::find_docsets;
// `dashtui list <dir>` prints every `*.docset` directory found under `<dir>`,
// one path per line, using `dashtui::docset::find_docsets`.
//
// Contract for this step (no `clap` yet):
//   - `std::env::args()` yields the binary path first, then the real args —
//     expect exactly two real args: the subcommand "list" and a directory.
//   - On success, print each found path, one per line, to stdout.
//   - On any failure (wrong/missing args, or `find_docsets` returning
//     `Err`), print a message to stderr and exit with a non-zero status.
//     (Look at `std::process::exit` for that last part.)
fn main() {
    let args: Vec<String> = args().collect();
    let exe = args.first().unwrap();
    match (args.get(1).map(String::as_str), args.get(2), args.get(3)) {
        (None, _, _) => invalid_call(exe, "No command given"),
        (Some(_), None, _) => invalid_call(exe, "No search path provided"),
        (Some("list"), Some(search), None) => {
            let search = Path::new(search);
            find_docsets(search)
                .unwrap_or_else(|e| {
                    eprintln!("An error occured: {}", e);
                    process::exit(1);
                })
                .iter()
                .for_each(|path| {
                    println!("{}", path.to_string_lossy());
                });
        }
        (Some(_), Some(_), None) => invalid_call(exe, "Unknown command"),
        (Some(_), Some(_), Some(_)) => invalid_call(exe, "Unexpected number of arguments"),
    };
}

fn invalid_call(exe: &str, err: &str) {
    eprintln!("{}. Usage: {} list <path>", err, exe);
    process::exit(1)
}
