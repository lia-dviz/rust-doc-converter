use crate::parser::UnifiedDocument;

pub fn export_docx(doc: &UnifiedDocument) -> String {
    format!("DOCX Export Structure for: {}", doc.title)
}
