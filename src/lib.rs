use std::fs;
use std::path::PathBuf;

pub fn list_jpg_files() -> Vec<PathBuf> {
    let dir = "/home/koushikk/Desktop";
    let mut jpg_files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("jpg") {
                    jpg_files.push(path);
                }
            }
        }
    }

    jpg_files
}

pub fn list_cbz_files() -> Vec<PathBuf> {
    let dir = "/home/koushikk/MANGA/Usogui/";

    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for e in entries.flatten() {
            let path = e.path();
            if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("cbz") {
                    files.push(path);
                }
            }
        }
    }
    files.sort();
    files
}

//need to create a function which would collect the cbz files, and then unwrap into each folder
//respectively, or i could just put it into memory but lowk f that tbh
