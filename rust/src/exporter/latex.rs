use crate::parser::UnifiedDocument;

pub fn export_latex(doc: &UnifiedDocument) -> String {
    let mut out = format!("\\documentclass{{article}}\n\\begin{{document}}\n\\title{{{}}}\n\\maketitle\n", doc.title);
    for node in &doc.nodes {
        out.push_str(&format!("{}\n\n", node.content));
    }
    out.push_str("\\end{document}");
    out
}
