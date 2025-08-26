use wasm_bindgen::JsCast;
use web_sys::{HtmlTextAreaElement, InputEvent};

pub fn textarea_value(e: InputEvent) -> String {
    e.target()
        .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
        .map(|input| input.value())
        .unwrap_or_default()
}
// Force use \n
pub fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n").replace('\r', "\n")
}

pub fn remove_trailing_blank_lines(s: &str) -> String {
    let trimmed = s.trim_end_matches('\n');
    trimmed.to_string()
}
