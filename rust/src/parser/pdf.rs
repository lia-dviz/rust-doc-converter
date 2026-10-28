use super::{UnifiedDocument, DocumentNode};

pub fn parse_pdf(path: &str) -> UnifiedDocument {
    UnifiedDocument {
        title: format!("PDF: {}", path),
        nodes: vec![
            DocumentNode {
                node_type: "paragraph".to_string(),
                content: "Conteúdo extraído nativamente do PDF.".to_string(),
                metadata: vec![("source".to_string(), path.to_string())],
            }
        ],
        assets: vec![],
    }
}
