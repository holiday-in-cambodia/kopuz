use std::path::Path;

pub fn format_artwork_url(path: Option<&impl AsRef<Path>>) -> Option<String> {
    const FRAGMENT: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'<')
        .add(b'>')
        .add(b'`');

    path.map(|p| {
        let p = p.as_ref();
        let p_str = p.to_string_lossy();
        let abs_path = if p_str.starts_with("./") {
            std::env::current_dir()
                .unwrap_or_default()
                .join(&p_str[2..])
        } else {
            p.to_path_buf()
        };

        format!(
            "artwork://local{}",
            percent_encoding::utf8_percent_encode(&abs_path.to_string_lossy(), FRAGMENT)
        )
    })
}
