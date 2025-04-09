use std::{
    env,
    fs::{self, DirEntry, File},
    io::{self, Write},
    path::Path,
};
use tl::parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() < 2 as usize {
        eprintln!("No arguments provided!");
        return;
    }
    match args[1].to_lowercase().as_str() {
        "build" => {
            if let Err(e) = build() {
                eprintln!("Error occured while trying to build! - {} -", e)
            }
        }
        "create" => {
            if args.len() < 3 as usize {
                eprintln!("No 2nd argument provided!");
                return;
            } else if args.len() < 4 as usize {
                eprintln!("No 3nd argument provided!");
                return;
            }
            match args[2].to_lowercase().as_str() {
                "page" => {
                    create_page(args[3].to_string());
                }
                "project" => {
                    if let Err(e) = create_project(args[3].to_string()) {
                        eprintln!("Error occured while running this command! - {} -", e)
                    }
                }
                "component" => {
                    if let Err(e) = create_component(args[3].to_string()) {
                        eprintln!("Error occured while running this command! - {} -", e)
                    }
                }
                _ => {
                    return;
                }
            }
        }
        _ => {
            eprintln!("Command not recognized! - {} -", args[1]);
            return;
        }
    }
}

fn build() -> std::io::Result<()> {
    println!("Build started!");
    let current_dir = env::current_dir()?;
    visit_dirs(&current_dir.join("pages"), &get_page)?;
    visit_dirs(&current_dir.join("components"), &perform_extension_action)?;
    Ok(())
}
fn create_component(component_name: String) -> std::io::Result<()> {
    let config_path = env::current_dir()?.join("blazing-config.json");
    if !config_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Blazing config not found inside this directory!",
        ));
    }
    let com_dir = env::current_dir()?.join("components");
    let new_dir = com_dir.join(&component_name);

    fs::create_dir(&new_dir)?;

    let mut html_file = File::create(new_dir.join(format!("{}.component.html", &component_name)))?;
    let mut js_file = File::create(new_dir.join(format!("{}.component.js", &component_name)))?;
    File::create(new_dir.join(format!("{}.component.css", &component_name)))?;
    let html_text = format!("{} works", component_name);
    let js_text = format!(
        r#"@Component({{
  selector: 'com-{}',
  templateUrl: './{}.component.html',
  styleUrls: ['./{}.component.css']
}})
  
  class {}Component{{
  
  }}"#,
        component_name, component_name, component_name, component_name
    );
    html_file.write(html_text.as_bytes())?;
    js_file.write(js_text.as_bytes())?;
    println!("Component created! {}", component_name);
    return Ok(());
}
fn create_page(page_name: String) {
    println!("Page created! {}", page_name)
}
fn create_project(project_name: String) -> std::io::Result<()> {
    let base_path = env::current_dir()?.join(&project_name);
    println!("Running from: {}", base_path.display());
    fs::create_dir(&base_path)?;

    let com_dir = base_path.join("components");
    let page_dir = base_path.join("pages");
    let out_dir = base_path.join("output");
    let index_path = page_dir.join("index.html");

    fs::create_dir(com_dir)?;
    fs::create_dir(&page_dir)?;
    File::create(index_path)?;
    fs::create_dir(out_dir)?;

    let config_path = base_path.join("blazing-config.json");
    let mut config_file = File::create(config_path)?;
    let config_text = format!(
        r#"{{
  "project_name": "{}",
  "output_dir": "output",
  "components_dir": "components",
  "pages_dir": "pages"
}}"#,
        project_name
    );
    config_file.write(config_text.as_bytes())?;

    return Ok(());
}
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)->std::io::Result<()>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }
    Ok(())
}
fn get_page(entry: &DirEntry) -> std::io::Result<()> {
    if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
        match ext {
            "html" => {
                let html = fs::read_to_string(entry.path())?;
                let document = parse(&html, tl::ParserOptions::default());
            }
            _ => {}
        }
    }
    return Ok(());
}
fn perform_extension_action(entry: &DirEntry) ->std::io::Result<()>{
    if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
        match ext {
            "js" => {}
            "html" => {}
            "css" => {}
            _ => {}
        }
    }
    return Ok(());
}