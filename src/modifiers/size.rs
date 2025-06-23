use quick_xml::{
    events::Event,
    Reader, Writer,
};

use std::io::Cursor;

pub fn update_size_bytes(input: &[u8], new_value: &str) -> std::io::Result<Vec<u8>> {
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