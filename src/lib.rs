use pulldown_cmark::{html, Parser};
use wasm_bindgen::prelude::*;

// Exported function that converts Markdown input to HTML
#[wasm_bindgen]
pub fn convert_markdown(markdown_input: &str) -> String {
    let parser = Parser::new(markdown_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

