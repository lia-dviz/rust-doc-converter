use crate::parser::UnifiedDocument;
use std::fs::File;
use std::io::{Read, Write};
use zip::write::FileOptions;
use zip::ZipArchive;

pub fn save_bundle(doc: &UnifiedDocument, output_path: &str) -> Result<(), String> {
    let file = File::create(output_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    zip.start_file("document.json", options).map_err(|e| e.to_string())?;
    let json = serde_json::to_string(doc).map_err(|e| e.to_string())?;
    zip.write_all(json.as_bytes()).map_err(|e| e.to_string())?;

    zip.finish().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn read_bundle(bundle_path: &str) -> Result<UnifiedDocument, String> {
    let file = File::open(bundle_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
    let mut json_file = archive.by_name("document.json").map_err(|e| e.to_string())?;
    let mut json_content = String::new();
    json_file.read_to_string(&mut json_content).map_err(|e| e.to_string())?;
    let doc: UnifiedDocument = serde_json::from_str(&json_content).map_err(|e| e.to_string())?;
    Ok(doc)
}
