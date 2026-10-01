//! syn_walker — Rust subprocess for Python-side EvidencePack collectors.
//!
//! Subcommands:
//!   structs   --root <project>     Walk all <project>/src/**.rs, emit
//!                                  one JSON object per `struct` definition.
//!
//!   allocs    --root <project>     Walk all <project>/src/**.rs, emit
//!                                  one JSON object per heap-alloc-site:
//!                                  `Vec::new`, `Vec::with_capacity`,
//!                                  `Box::new`, `vec!`, `String::new`,
//!                                  `String::with_capacity`.
//!
//! Output: a single JSON document on stdout (NOT line-delimited) of the
//! form `{"items": [...]}`. Errors go to stderr with non-zero exit.
//!
//! Robustness: a parse failure on one file is logged to stderr but does
//! not abort the walk — Python receives whatever was successfully parsed.

use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use syn::visit::Visit;
use walkdir::WalkDir;

mod allocs;
mod loop_invariant;
mod structs;

#[derive(Serialize)]
struct Output<T: Serialize> {
    project_root: String,
    files_scanned: usize,
    files_parse_failed: Vec<String>,
    items: Vec<T>,
}

fn print_usage() {
    eprintln!(
        "syn_walker — Rust AST walker for EvidencePack v1.2 collectors\n\
         \n\
         USAGE:\n\
             syn_walker structs              --root <project>\n\
             syn_walker allocs               --root <project>\n\
             syn_walker loop-invariant-conds --root <project>\n\
         \n\
         The project must contain a `src/` directory with Rust source files.\n\
         Output is a single JSON document on stdout."
    );
}

fn parse_args() -> Option<(String, PathBuf)> {
    let argv: Vec<String> = std::env::args().collect();
    // [0]=binary, [1]=subcommand, [2..]=flags
    if argv.len() < 4 {
        return None;
    }
    let subcmd = argv[1].clone();
    let mut root: Option<PathBuf> = None;
    let mut i = 2;
    while i < argv.len() {
        match argv[i].as_str() {
            "--root" if i + 1 < argv.len() => {
                root = Some(PathBuf::from(&argv[i + 1]));
                i += 2;
            }
            _ => return None,
        }
    }
    root.map(|r| (subcmd, r))
}

fn collect_rust_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let src = root.join("src");
    let walk_root = if src.is_dir() { src } else { root.to_path_buf() };
    for entry in WalkDir::new(&walk_root).follow_links(false) {
        let Ok(entry) = entry else { continue };
        if entry.file_type().is_file()
            && entry.path().extension().and_then(|e| e.to_str()) == Some("rs")
        {
            files.push(entry.path().to_path_buf());
        }
    }
    files.sort();
    files
}

fn run_structs(root: &Path) -> Output<structs::StructItem> {
    let files = collect_rust_files(root);
    let mut all = Vec::new();
    let mut parse_failed = Vec::new();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else {
            parse_failed.push(path.display().to_string());
            continue;
        };
        match syn::parse_file(&text) {
            Ok(file) => {
                let mut v = structs::Walker::new(path, &text);
                v.visit_file(&file);
                all.extend(v.into_items());
            }
            Err(e) => {
                eprintln!("[syn_walker] parse failed for {}: {}", path.display(), e);
                parse_failed.push(path.display().to_string());
            }
        }
    }
    Output {
        project_root: root.display().to_string(),
        files_scanned: files.len(),
        files_parse_failed: parse_failed,
        items: all,
    }
}

fn run_allocs(root: &Path) -> Output<allocs::AllocSite> {
    let files = collect_rust_files(root);
    let mut all = Vec::new();
    let mut parse_failed = Vec::new();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else {
            parse_failed.push(path.display().to_string());
            continue;
        };
        match syn::parse_file(&text) {
            Ok(file) => {
                let mut v = allocs::Walker::new(path, &text);
                v.visit_file(&file);
                all.extend(v.into_items());
            }
            Err(e) => {
                eprintln!("[syn_walker] parse failed for {}: {}", path.display(), e);
                parse_failed.push(path.display().to_string());
            }
        }
    }
    Output {
        project_root: root.display().to_string(),
        files_scanned: files.len(),
        files_parse_failed: parse_failed,
        items: all,
    }
}

fn run_loop_invariant_conds(root: &Path) -> Output<loop_invariant::LoopInvariantCond> {
    let files = collect_rust_files(root);
    let mut all = Vec::new();
    let mut parse_failed = Vec::new();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else {
            parse_failed.push(path.display().to_string());
            continue;
        };
        match syn::parse_file(&text) {
            Ok(file) => {
                let mut v = loop_invariant::Walker::new(path, &text);
                v.visit_file(&file);
                all.extend(v.into_items());
            }
            Err(e) => {
                eprintln!("[syn_walker] parse failed for {}: {}", path.display(), e);
                parse_failed.push(path.display().to_string());
            }
        }
    }
    Output {
        project_root: root.display().to_string(),
        files_scanned: files.len(),
        files_parse_failed: parse_failed,
        items: all,
    }
}

fn main() -> ExitCode {
    let Some((subcmd, root)) = parse_args() else {
        print_usage();
        return ExitCode::from(2);
    };

    if !root.is_dir() {
        eprintln!("[syn_walker] --root not a directory: {}", root.display());
        return ExitCode::from(2);
    }

    let json = match subcmd.as_str() {
        "structs" => serde_json::to_string_pretty(&run_structs(&root)),
        "allocs"  => serde_json::to_string_pretty(&run_allocs(&root)),
        "loop-invariant-conds" => serde_json::to_string_pretty(&run_loop_invariant_conds(&root)),
        other => {
            eprintln!("[syn_walker] unknown subcommand: {}", other);
            print_usage();
            return ExitCode::from(2);
        }
    };

    match json {
        Ok(s) => {
            println!("{}", s);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("[syn_walker] JSON serialisation failed: {}", e);
            ExitCode::from(1)
        }
    }
}
