use crate::parser::UnifiedDocument;

pub fn export_html(doc: &UnifiedDocument) -> String {
    let mut out = format!("<!DOCTYPE html>\n<html>\n<head><title>{}</title></head>\n<body>\n<h1>{}</h1>\n", doc.title, doc.title);
    for node in &doc.nodes {
        out.push_str(&format!("<p>{}</p>\n", node.content));
    }
    out.push_str("</body>\n</html>");
    out
}
