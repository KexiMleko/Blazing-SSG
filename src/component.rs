use std::path::PathBuf;

pub struct ComponentBl {
    pub selector: String,
    pub class_name: String,
    pub html_path: PathBuf,
    pub js_path: PathBuf,
    pub css_paths: Vec<PathBuf>,
}
impl ComponentBl {
    pub fn to_string(&self) -> String {
        let css_paths = self
            .css_paths
            .iter()
            .map(|p| format!("    {}", p.display()))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "ComponentBl {{\n\
             selector:   {}\n\
             class_name: {}\n\
             html_path:  {}\n\
             js_path:    {}\n\
             css_paths:\n{}\n\
             }}",
            self.selector,
            self.class_name,
            self.html_path.display(),
            self.js_path.display(),
            css_paths
        )
    }
}
