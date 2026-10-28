mod parser;
mod exporter;
mod bundle;

use pyo3::prelude::*;
use parser::{pdf, office, opendoc};
use exporter::{markdown, html, latex, docx};
use bundle::{save_bundle, read_bundle};

#[pyfunction]
fn parse_and_convert(input_path: String, output_format: String, bundle_path: Option<String>) -> PyResult<String> {
    let doc = if input_path.ends_with(".bundle") {
        read_bundle(&input_path).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?
    } else {
        let parsed = if input_path.ends_with(".pdf") {
            pdf::parse_pdf(&input_path)
        } else if input_path.ends_with(".docx") || input_path.ends_with(".xlsx") || input_path.ends_with(".pptx") {
            office::parse_office(&input_path)
        } else {
            opendoc::parse_opendoc(&input_path)
        };

        if let Some(ref b_path) = bundle_path {
            let _ = save_bundle(&parsed, b_path);
        }
        parsed
    };

    match output_format.as_str() {
        "markdown" | "md" => Ok(markdown::export_markdown(&doc)),
        "html" => Ok(html::export_html(&doc)),
        "latex" => Ok(latex::export_latex(&doc)),
        "docx" => Ok(docx::export_docx(&doc)),
        "json" => Ok(serde_json::to_string(&doc).unwrap_or_default()),
        _ => Ok(markdown::export_markdown(&doc)),
    }
}

#[pymodule]
fn doc_converter_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_and_convert, m)?)?;
    Ok(())
}
