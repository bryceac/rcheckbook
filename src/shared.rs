use url_path::UrlPath;

pub fn real_path(p: &str) -> String {
    if p.starts_with("~") {
        shellexpand::tilde(p).into_owned()
    } else {
        UrlPath::new(p).normalize()
    }
}