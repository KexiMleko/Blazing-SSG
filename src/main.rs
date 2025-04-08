use std::{
    env,
    fs::{self, File},
    io::{self, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() < 2 as usize {
        eprintln!("No arguments provided!");
        return;
    }
    match args[1].to_lowercase().as_str() {
        "build" => {
            build();
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

fn build() {
    println!("Build started!");
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

    let mut html_file = File::create(new_dir.join(format!("{}-component.html", &component_name)));
    let mut js_file = File::create(new_dir.join(format!("{}-component.js", &component_name)));
    let mut css_file = File::create(new_dir.join(format!("{}-component.css", &component_name)));
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
  "project_name": {},
  "output_dir": "output",
  "components_dir": "components",
  "pages_dir": "pages"
}}"#,
        project_name
    );
    config_file.write(config_text.as_bytes())?;

    return Ok(());
}
