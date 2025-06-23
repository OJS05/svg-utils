use quick_xml::{
    events::Event,
    Reader, Writer,
};

use std::io::Cursor;

pub fn update_stroke_width_bytes(input: &[u8], new_value: &str) -> std::io::Result<Vec<u8>> {
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