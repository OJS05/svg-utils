mod modifiers {
    pub mod size;
    pub mod stroke_width;
    pub mod colour;
}

use walkdir::WalkDir;
use modifiers::stroke_width::update_stroke_width_bytes;
use modifiers::colour::update_colour_bytes;
use modifiers::size::update_size_bytes;
use std::{
    env,
    fs::File,
    io::{Read, Write},
    path::Path
};

fn main() {
    let mut input = String::new();

    println!("Enter the size to set for SVG files (e.g., 256):");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let size = input.trim().to_string();
    input.clear();

    println!("Enter the stroke-width to set for SVG files (e.g., 1):");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let stroke_width = input.trim().to_string();
    input.clear();

    println!("Enter the stroke color to set for SVG files (e.g., currentColor):");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let stroke_color = input.trim().to_string();
    println!("Processing SVG files with size: {}, stroke-width: {}, stroke color: {}", size, stroke_width, stroke_color);



    // Get the current working directory
    let input_path = env::current_exe().unwrap().parent().expect("Error with directory").join("input");

    traverse(&input_path, &size, &stroke_width, &stroke_color);
    println!("SVG processing complete. Check the 'output' directory for results.");
}

fn traverse(input_root: &Path, size: &str, stroke_width: &str, stroke_color: &str) {
    let output_root = env::current_exe().unwrap().parent().expect("Error with output in traverse").join("output");
    println!("{}", input_root.display());
    for entry in WalkDir::new(input_root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {continue};
        if entry.path().extension().and_then(|s| s.to_str()) != Some("svg") {
            continue;
        }

        let rel_path = entry.path().strip_prefix(input_root).expect("problem with relpath");
        let output_path = output_root.join(rel_path);

        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent).expect("problem creating parent directories");
        }

        let mut input_data = Vec::new();
        File::open(entry.path()).unwrap().read_to_end(&mut input_data).unwrap();

        let data = match update_size_bytes(&input_data, size) {
            Ok(d) => d,
            Err(e) => {
                println!("Failed to update size for {}: {}", entry.path().display(), e);
                continue;
            }
        };
        let data = match update_stroke_width_bytes(&data, stroke_width) {
            Ok(d) => d,
            Err(e) => {
                println!("Failed to update stroke-width for {}: {}", entry.path().display(), e);
                continue;
            }
        };
        let data = match update_colour_bytes(&data, stroke_color) {
            Ok(d) => d,
            Err(e) => {
                println!("Failed to update colour for {}: {}", entry.path().display(), e);
                continue;
            }
        };

        let mut output_file = File::create(&output_path).expect("problem creating output file");
        output_file.write_all(&data).expect("problem writing to output file");
    }
}