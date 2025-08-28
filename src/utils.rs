use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlTextAreaElement};
use yew_hooks::UseClipboardHandle;
use yew::prelude::*;
use gloo_dialogs::alert;

pub fn input_value(t: Option<EventTarget>) -> String {
    t.and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
        .map(|input| input.value())
        .unwrap_or_default()
}

pub fn textarea_value(t: Option<EventTarget>) -> String {
    t.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
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

pub fn make_write_to_clipboard_btn(
    clipboard: UseClipboardHandle,
    text: String,
) -> Html {
    html! {
        <button class="btn-clipboard" onclick={
            (!text.is_empty()).then_some(Callback::from(move |_| {
                clipboard.write_text(text.clone());
                alert("复制成功");
            }))
        }>
            { "复制" }
        </button>
    }
}
