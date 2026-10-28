use super::{UnifiedDocument, DocumentNode};
use std::fs::File;
use std::io::Read;
use zip::ZipArchive;

pub fn parse_opendoc(path: &str) -> UnifiedDocument {
    let mut nodes = vec![];
    let file = File::open(path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    if let Ok(mut xml_file) = archive.by_name("content.xml") {
        let mut content = String::new();
        let _ = xml_file.read_to_string(&mut content);
        nodes.push(DocumentNode {
            node_type: "opendoc_xml".to_string(),
            content: format!("Extraído de OpenDocument ({} bytes)", content.len()),
            metadata: vec![("format".to_string(), "opendoc".to_string())],
        });
    }

    UnifiedDocument {
        title: format!("OpenDoc: {}", path),
        nodes,
        assets: vec![],
    }
}
