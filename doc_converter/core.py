from . import doc_converter_core

def process_document(input_path: str, output_format: str, bundle_path: str = None) -> str:
    return doc_converter_core.parse_and_convert(input_path, output_format, bundle_path)
