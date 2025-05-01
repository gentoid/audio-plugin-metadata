use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub fn scan_path<'a>(root: &'a Path, extensions: &'a [&str]) -> impl Iterator<Item = PathBuf> + 'a {
    WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter_map(move |entry| {
            let ext = entry.path().extension()?.to_str()?;
            extensions
                .iter()
                .any(|wanted| wanted.eq_ignore_ascii_case(ext))
                .then(|| entry.into_path())
        })
}
