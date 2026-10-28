use crate::parser::UnifiedDocument;

pub fn export_markdown(doc: &UnifiedDocument) -> String {
    let mut out = format!("# {}\n\n", doc.title);
    for node in &doc.nodes {
        out.push_str(&node.content);
        out.push_str("\n\n");
    }
    out
}
