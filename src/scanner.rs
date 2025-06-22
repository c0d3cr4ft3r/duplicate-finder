use std::collections::HashMap;
use std::path::PathBuf;
use walkdir::WalkDir;
use rayon::prelude::*;
use crate::model::DuplicateRow;
use crate::hash::hash_file;

pub fn scan_for_duplicates(root: PathBuf) -> Vec<DuplicateRow> {
    let all_files: Vec<_> = WalkDir::new(&root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();

    let mut size_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    for path in all_files {
        if let Ok(meta) = path.metadata() {
            size_map.entry(meta.len()).or_default().push(path);
        }
    }

    let candidates: Vec<_> = size_map
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .flat_map(|(size, files)| files.into_iter().map(move |f| (size, f)))
        .collect();

    let hash_map: HashMap<String, Vec<(u64, PathBuf)>> = candidates
        .par_iter()
        .filter_map(|(size, path)| {
            hash_file(path).ok().map(|hash| (hash, (*size, path.clone())))
        })
        .collect::<Vec<_>>()
        .into_par_iter()
        .fold(HashMap::new, |mut acc, (hash, pair)| {
            acc.entry(hash).or_insert_with(Vec::new).push(pair);
            acc
        })
        .reduce(HashMap::new, |mut a, b| {
            for (hash, paths) in b {
                a.entry(hash).or_insert_with(Vec::new).extend(paths);
            }
            a
        });

    let mut rows = Vec::new();
    for (hash, group) in hash_map {
        if group.len() < 2 {
            continue;
        }
        for (size, path) in group {
            rows.push(DuplicateRow {
                hash: hash.clone(),
                size,
                path: path.display().to_string(),
            });
        }
    }

    rows
}
