use std::fs;
use std::path::{Path, PathBuf};

fn detect_image_extension(data: &[u8]) -> &'static str {
    if data.len() >= 12 && &data[..8] == b"\x89PNG\r\n\x1a\n" {
        "png"
    } else if data.len() >= 3 && data[..3] == [0xFF, 0xD8, 0xFF] {
        "jpg"
    } else if data.len() >= 12 && &data[..4] == b"RIFF" && &data[8..12] == b"WEBP" {
        "webp"
    } else if data.len() >= 6 && (data[..6] == *b"GIF87a" || data[..6] == *b"GIF89a") {
        "gif"
    } else if data.len() >= 2 && data[..2] == [0x42, 0x4D] {
        "bmp"
    } else {
        "jpg"
    }
}

pub fn find_folder_cover(dir: &Path) -> Option<PathBuf> {
    let candidates = ["cover.jpg", "cover.png", "folder.jpg", "album.jpg"];

    for name in candidates {
        let p = dir.join(name);
        if p.exists() {
            return Some(p);
        }
    }
    None
}

pub fn save_cover(album_id: &str, data: &[u8], cache_dir: &Path) -> std::io::Result<PathBuf> {
    fs::create_dir_all(cache_dir)?;
    let extension = detect_image_extension(data);
    let path = cache_dir.join(format!("{album_id}.{extension}"));

    fs::write(&path, data)?;
    Ok(path)
}
