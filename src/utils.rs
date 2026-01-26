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
        if p_str.starts_with("./") {
            let abs_path = std::env::current_dir()
                .unwrap_or_default()
                .join(&p_str[2..]);
            format!("artwork://local{}", abs_path.to_string_lossy())
        } else {
            format!("artwork://{}", percent_encoding::utf8_percent_encode(&p_str, FRAGMENT))
        }
    })
}
