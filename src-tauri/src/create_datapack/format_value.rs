use serde::Serialize;

pub fn format_value(json: serde_json::Value) -> String {
    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    
    json.serialize(&mut ser).unwrap();

    return String::from_utf8(buf).unwrap();
}