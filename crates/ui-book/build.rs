use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/stories/");
    println!("cargo:rerun-if-changed=../ui/src/visual/");

    let out_dir = env::var("OUT_DIR").unwrap();
    let stories_dir = Path::new(&out_dir).join("stories");
    fs::create_dir_all(&stories_dir).expect("Failed to create stories output directory");

    // Parse story files and map them to their components
    let story_component_map = parse_story_files();

    // Generate individual files for each story
    for (story_name, component_info) in &story_component_map {
        if let Ok(source_code) = read_component_source(&component_info.file_path) {
            let file_name = format!("{story_name}_source.rs");
            let file_path = stories_dir.join(file_name);

            let mut generated_code = String::new();
            generated_code.push_str(&format!(
                "// Auto-generated source code for {story_name} component\n\n"
            ));

            let const_name = format!("{}_SOURCE", story_name.to_uppercase());
            generated_code.push_str(&format!(
                "const {const_name}: &str = r###\"\n```rust\n{source_code}\n```\n\"###;\n\n"
            ));

            fs::write(&file_path, generated_code)
                .unwrap_or_else(|_| panic!("Failed to write source file for {story_name}"));
        }
    }
}

#[derive(Debug)]
struct ComponentInfo {
    file_path: String,
}

fn parse_story_files() -> HashMap<String, ComponentInfo> {
    let mut map = HashMap::new();

    let stories_dir = Path::new("src/stories");
    if let Ok(entries) = fs::read_dir(stories_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                let file_name = path.file_stem().unwrap().to_str().unwrap();
                if file_name != "mod" {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let components = extract_holt_ui_imports(&content);
                        if !components.is_empty() {
                            let component_path = format!("../ui/src/visual/{file_name}.rs");
                            map.insert(
                                file_name.to_string(),
                                ComponentInfo {
                                    file_path: component_path,
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    map
}

fn extract_holt_ui_imports(content: &str) -> Vec<String> {
    let mut components = Vec::new();

    for line in content.lines() {
        let line = line.trim();

        // Look for imports from holt_ui::visual
        if line.starts_with("use holt_ui::visual::") {
            // Parse: use holt_ui::visual::{Badge, BadgeVariant};
            if let Some(imports_part) = line.strip_prefix("use holt_ui::visual::") {
                if imports_part.starts_with('{') && imports_part.contains('}') {
                    // Extract content between braces
                    if let Some(start) = imports_part.find('{') {
                        if let Some(end) = imports_part.find('}') {
                            let imports = &imports_part[start + 1..end];
                            for import in imports.split(',') {
                                let import = import.trim();
                                // Only take the main component name, not variants
                                if !import.contains("Variant")
                                    && !import.contains("Size")
                                    && !import.is_empty()
                                {
                                    components.push(import.to_string());
                                }
                            }
                        }
                    }
                } else {
                    // Single import: use holt_ui::visual::Badge;
                    let import = imports_part.trim_end_matches(';').trim();
                    if !import.contains("Variant") && !import.contains("Size") && !import.is_empty()
                    {
                        components.push(import.to_string());
                    }
                }
            }
        }
    }

    components.sort();
    components.dedup();
    components
}

fn read_component_source(component_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(component_path)
}
