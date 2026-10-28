pub mod pdf;
pub mod office;
pub mod opendoc;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocumentNode {
    pub node_type: String,
    pub content: String,
    pub metadata: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnifiedDocument {
    pub title: String,
    pub nodes: Vec<DocumentNode>,
    pub assets: Vec<String>,
}
