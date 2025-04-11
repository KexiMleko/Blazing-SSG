use std::path::PathBuf;

pub struct ComponentBl {
    pub selector: String,
    pub class_name: String,
    pub html_path: PathBuf,
    pub js_path: PathBuf,
    pub css_paths: Vec<PathBuf>,
}
