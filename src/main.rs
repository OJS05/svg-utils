use walkdir::WalkDir;
use std::{
    env,
    fs::File,
    io::{Read, Write, Cursor},
    path::{Path, PathBuf},
};
use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Reader, Writer,
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

/// Recursively processes all files in the given directory, applying SVG changes and preserving directory structure.
fn traverse(input_root: &Path, size: &str, stroke_width: &str, stroke_color: &str) {
    let output_root = env::current_exe().unwrap().parent().expect("Error with output in traverse").join("output");
    println!("{}", input_root.display());
    for entry in WalkDir::new(input_root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {continue};
        if entry.path().extension().and_then(|s| s.to_str()) != Some("svg") {
            continue;
        }
        // Compute the relative path from the input root
        let rel_path = entry.path().strip_prefix(input_root).expect("problem with relpath");
        let output_path = output_root.join(rel_path);

        // Create parent directories if needed
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent).expect("problem creating parent directories");
        }

        // Read the input file into memory
        let mut input_data = Vec::new();
        File::open(entry.path()).unwrap().read_to_end(&mut input_data).unwrap();

        // Apply changes incrementally in memory
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

        // Write the final result to the output file
        let mut output_file = File::create(&output_path).expect("problem creating output file");
        output_file.write_all(&data).expect("problem writing to output file");
    }
}

/// Update "stroke-width" attributes in SVG/XML bytes.
fn update_stroke_width_bytes(input: &[u8], new_value: &str) -> std::io::Result<Vec<u8>> {
    let mut reader = Reader::from_reader(Cursor::new(input));
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let mut elem = e.to_owned();
                elem.clear_attributes();
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"stroke-width" {
                        elem.push_attribute(("stroke-width", new_value));
                    } else {
                        elem.push_attribute(attr);
                    }
                }
                writer.write_event(Event::Start(elem))?;
            }
            Ok(Event::Empty(ref e)) => {
                let mut elem = e.to_owned();
                elem.clear_attributes();
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"stroke-width" {
                        elem.push_attribute(("stroke-width", new_value));
                    } else {
                        elem.push_attribute(attr);
                    }
                }
                writer.write_event(Event::Empty(elem))?;
            }
            Ok(Event::End(ref e)) => {
                writer.write_event(Event::End(e.to_owned()))?;
            }
            Ok(Event::Text(ref e)) => {

                if e.as_ref().windows(b"stroke-width".len()).any(|w| w == b"stroke-width") {
                    let orig_text = e.unescape().unwrap_or_else(|_| "".into());
                    let replaced = replace_stroke_width_in_css(&orig_text, new_value);
                    writer.write_event(Event::Text(quick_xml::events::BytesText::from_escaped(&replaced)))?;
                } else {
                    writer.write_event(Event::Text(e.to_owned()))?;
                }
            }
            Ok(Event::Eof) => break,
            Ok(e) => {
                writer.write_event(e)?;
            }
            Err(_) => break,
        }
        buf.clear();
    }

    Ok(writer.into_inner().into_inner())
}

fn replace_stroke_width_in_css(text: &str, new_value: &str) -> String {
    let re = regex::Regex::new(r"(stroke-width\s*:\s*)([^;]+)").unwrap();
    // println!("Replacing stroke-width in CSS: {}", text);
    // println!("New value: {}", new_value);
    // println!("Regex: {}", re);
    re.replace_all(text, format!("${{1}}{}", new_value)).to_string()
}

/// Update "stroke" attributes in SVG/XML bytes.
fn update_colour_bytes(input: &[u8], new_value: &str) -> std::io::Result<Vec<u8>> {
    let mut reader = Reader::from_reader(Cursor::new(input));
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let mut elem = e.to_owned();
                elem.clear_attributes();
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"stroke" {
                        elem.push_attribute(("stroke", new_value));
                    } else {
                        elem.push_attribute(attr);
                    }
                }
                writer.write_event(Event::Start(elem))?;
            }
            Ok(Event::Empty(ref e)) => {
                let mut elem = e.to_owned();
                elem.clear_attributes();
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"stroke" {
                        elem.push_attribute(("stroke", new_value));
                    } else {
                        elem.push_attribute(attr);
                    }
                }
                writer.write_event(Event::Empty(elem))?;
            }
            Ok(Event::End(ref e)) => {
                writer.write_event(Event::End(e.to_owned()))?;
            }
            Ok(Event::Text(ref e)) => {
                if e.as_ref().windows(b"stroke".len()).any(|w| w == b"stroke") {
                    let orig_text = e.unescape().unwrap_or_else(|_| "".into());
                    let replaced = replace_stroke_color_in_css(&orig_text, new_value);
                    writer.write_event(Event::Text(quick_xml::events::BytesText::from_escaped(&replaced)))?;
                } else {
                    writer.write_event(Event::Text(e.to_owned()))?;
                }
            }
            Ok(Event::Eof) => break,
            Ok(e) => {
                writer.write_event(e)?;
            }
            Err(_) => break,
        }
        buf.clear();
    }

    Ok(writer.into_inner().into_inner())
}

fn replace_stroke_color_in_css(text: &str, new_value: &str) -> String {
    let re = regex::Regex::new(r"(stroke\s*:\s*)([^;]+)").unwrap();
    re.replace_all(text, format!("${{1}}{}", new_value)).to_string()
}

/// Update "width" and "height" attributes on the root <svg> element in SVG/XML bytes.
fn update_size_bytes(input: &[u8], new_value: &str) -> std::io::Result<Vec<u8>> {
    let mut reader = Reader::from_reader(Cursor::new(input));
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let mut elem = e.to_owned();

                if e.name().as_ref() != b"svg" {
                    writer.write_event(Event::Start(elem))?;
                    continue;
                }

                elem.clear_attributes();
                for attr in e.attributes().flatten() {
                    elem.push_attribute(("height", new_value));
                    elem.push_attribute(("width", new_value));
                    elem.push_attribute(attr);
                }

                writer.write_event(Event::Start(elem))?;
            }
            Ok(Event::Empty(ref e)) => {
                let mut elem = e.to_owned();

                if e.name().as_ref() != b"svg" {
                    writer.write_event(Event::Empty(elem))?;
                    continue;
                }

                elem.clear_attributes();
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"height" {
                        elem.push_attribute(("height", new_value));
                    } else if attr.key.as_ref() == b"width" {
                        elem.push_attribute(("width", new_value));
                    } else {
                        elem.push_attribute(attr);
                    }
                }

                writer.write_event(Event::Empty(elem))?;
            }
            Ok(Event::End(ref e)) => {
                writer.write_event(Event::End(e.to_owned()))?;
            }
            Ok(Event::Text(ref e)) => {
                if e.as_ref().windows(b"width".len()).any(|w| w == b"width") ||
                   e.as_ref().windows(b"height".len()).any(|w| w == b"height") {
                    let orig_text = e.unescape().unwrap_or_else(|_| "".into());
                    let replaced = replace_size_in_css(&orig_text, new_value);
                    writer.write_event(Event::Text(quick_xml::events::BytesText::from_escaped(&replaced)))?;
                } else {
                    writer.write_event(Event::Text(e.to_owned()))?;
                }
            }
            Ok(Event::Eof) => break,
            Ok(e) => {
                writer.write_event(e)?;
            }
            Err(_) => break,
        }
        buf.clear();
    }

    Ok(writer.into_inner().into_inner())
}

fn replace_size_in_css(text: &str, new_value: &str) -> String {
    let re_width = regex::Regex::new(r"(width\s*:\s*)([^;]+)").unwrap();
    let re_height = regex::Regex::new(r"(height\s*:\s*)([^;]+)").unwrap();
    let text = re_width.replace_all(text, format!("${{1}}{}", new_value)).to_string();
    re_height.replace_all(&text, format!("${{1}}{}", new_value)).to_string()
}