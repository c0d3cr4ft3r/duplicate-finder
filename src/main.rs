mod cli;
mod hash;
mod model;
mod scanner;
mod preview;
mod interactive;

use clap::Parser;
use tabled::Table;
use std::collections::HashSet;
use std::path::PathBuf;
use crate::cli::Args;
use crate::scanner::scan_for_duplicates;
use crate::preview::open_with_system_viewer;
use crate::interactive::run_interactive;
use regex::Regex;

fn main() {
    let args = Args::parse();
    let root = PathBuf::from(&args.path);
    let rows = scan_for_duplicates(root);

    if rows.is_empty() {
        println!("No duplicates found.");
        return;
    }

    let table = Table::new(&rows).to_string();
    println!("{table}");

    if args.open {
        println!("\nOpening one file per duplicate set...");
        let mut opened = HashSet::new();
        for row in &rows {
            if opened.insert(&row.hash) {
                let _ = open_with_system_viewer(&row.path);
            }
        }
    }

    if args.interactive {
        run_interactive(&rows);
    } else if let Some(grouped) = group_by_hash(&rows) {
        println!("\nRunning smart deletion policy...");

        for (hash, group) in grouped {
            if group.len() < 2 {
                continue;
            }

            let keep_index = if args.keep_newest {
                group.iter().enumerate().max_by_key(|(_, r)| {
                    std::fs::metadata(&r.path).and_then(|m| m.modified()).ok()
                }).map(|(i, _)| i)
            } else if args.keep_oldest {
                group.iter().enumerate().min_by_key(|(_, r)| {
                    std::fs::metadata(&r.path).and_then(|m| m.modified()).ok()
                }).map(|(i, _)| i)
            } else if args.keep_shortest {
                group.iter().enumerate().min_by_key(|(_, r)| r.path.len()).map(|(i, _)| i)
            } else if let Some(pattern) = &args.keep_regex {
                let re = Regex::new(pattern).unwrap();
                group.iter().enumerate().find(|(_, r)| re.is_match(&r.path)).map(|(i, _)| i)
            } else {
                None
            };

            if let Some(keep) = keep_index {
                for (i, file) in group.iter().enumerate() {
                    if i != keep {
                        println!("Deleting: {}", file.path);
                        if let Err(e) = std::fs::remove_file(&file.path) {
                            eprintln!("⚠️  Failed to delete {}: {}", file.path, e);
                        }
                    }
                }
            }
        }

        println!("\n✅ Smart deletion complete.");
    }
}

fn group_by_hash(rows: &[crate::model::DuplicateRow]) -> Option<Vec<(String, Vec<&crate::model::DuplicateRow>)>> {
    let mut map = std::collections::HashMap::<String, Vec<&crate::model::DuplicateRow>>::new();
    for row in rows {
        map.entry(row.hash.clone()).or_default().push(row);
    }

    if map.is_empty() {
        None
    } else {
        Some(map.into_iter().collect())
    }
}
