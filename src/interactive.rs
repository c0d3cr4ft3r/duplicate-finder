use crate::model::DuplicateRow;
use crate::preview::open_with_system_viewer;
use std::collections::HashSet;
use std::io::{self, Write};
use std::fs;

pub fn run_interactive(rows: &[DuplicateRow]) {
    println!("\nInteractive deletion mode:");
    let mut processed = HashSet::new();

    for row in rows {
        if processed.insert(&row.hash) {
            let group: Vec<_> = rows
                .iter()
                .filter(|r| r.hash == row.hash)
                .collect();

            if group.len() <= 1 {
                continue;
            }

            println!("\nDuplicate group (hash: {}):", row.hash);
            for (i, item) in group.iter().enumerate() {
                println!("  [{}] {} ({} bytes)", i, item.path, item.size);
            }

            let mut selection: Option<usize> = None;
            let mut input = String::new();

            loop {
                println!("  [p] Preview a file");
                println!("  [k] Select file to KEEP");
                print!("> ");
                io::stdout().flush().unwrap();

                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let trimmed = input.trim();

                if trimmed == "p" {
                    print!("Enter file number to preview: ");
                    io::stdout().flush().unwrap();
                    input.clear();
                    io::stdin().read_line(&mut input).unwrap();
                    if let Ok(idx) = input.trim().parse::<usize>() {
                        if let Some(file) = group.get(idx) {
                            open_with_system_viewer(&file.path);
                        }
                    }
                } else if trimmed == "k" {
                    print!("Enter file number to KEEP: ");
                    io::stdout().flush().unwrap();
                    input.clear();
                    io::stdin().read_line(&mut input).unwrap();
                    if let Ok(idx) = input.trim().parse::<usize>() {
                        if idx < group.len() {
                            selection = Some(idx);
                            break;
                        }
                    }
                }
            }

            if let Some(to_keep) = selection {
                for (i, file) in group.iter().enumerate() {
                    if i != to_keep {
                        println!("Deleting: {}", file.path);
                        if let Err(err) = fs::remove_file(&file.path) {
                            eprintln!("  ⚠️  Failed to delete {}: {}", file.path, err);
                        }
                    }
                }
            }
        }
    }

    println!("\n✅ Cleanup complete.");
}
