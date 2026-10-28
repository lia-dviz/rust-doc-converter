use super::{UnifiedDocument, DocumentNode};
use std::fs::File;
use std::io::Read;
use zip::ZipArchive;

pub fn parse_office(path: &str) -> UnifiedDocument {
    let mut nodes = vec![];
    let file = File::open(path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    let target_file = if path.ends_with(".docx") {
        Some("word/document.xml")
    } else if path.ends_with(".xlsx") {
        Some("xl/worksheets/sheet1.xml")
    } else {
        None
    };

    if let Some(filename) = target_file {
        if let Ok(mut xml_file) = archive.by_name(filename) {
            let mut content = String::new();
            let _ = xml_file.read_to_string(&mut content);
            nodes.push(DocumentNode {
                node_type: "xml_raw".to_string(),
                content: format!("Trecho extraído do Office XML ({} bytes)", content.len()),
                metadata: vec![("internal_path".to_string(), filename.to_string())],
            });
        }
    }

    UnifiedDocument {
        title: format!("Office: {}", path),
        nodes,
        assets: vec![],
    }
}
