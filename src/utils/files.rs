use std::{fs, path::{Path, PathBuf}};

pub struct DirData {
    pub parent_path: PathBuf,
    pub filename: String
}

pub fn get_directories(path: &str) -> Vec<DirData> {
    let path = Path::new(path);

    if path.exists() {
        if let Ok(entries) = fs::read_dir(path) {
            return entries
                .filter_map(|entry| {
                    entry.ok().map(|e| DirData {
                        parent_path: e.path().parent().unwrap_or(path).canonicalize().unwrap_or(PathBuf::from("")),
                        filename: e.file_name().to_string_lossy().into_owned()
                    })
                }).collect()
        }
    }

    Vec::new()
}
