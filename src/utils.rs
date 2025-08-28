use std::rc::Rc;

use gloo_dialogs::alert;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{Clipboard, EventTarget, HtmlInputElement, HtmlTextAreaElement, window};
use yew::prelude::*;

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

// Some code is from yew-hooks use_clipboard.rs

pub fn get_clipboard() -> Clipboard {
    window()
        .expect_throw("Can't find the global Window")
        .navigator()
        .clipboard()
}

pub fn make_write_to_clipboard_btn(clipboard: Rc<Clipboard>, text: String) -> Html {
    html! {
        <button class="btn-clipboard" onclick={
            let text = text.clone();
            (!text.is_empty()).then_some(Callback::from(move |_| {
                let _ =clipboard.write_text(&text);
                alert("复制成功");
            }))
        }>
            { "复制" }
        </button>
    }
}

pub fn make_read_from_clipboard_btn(
    clipboard: Rc<Clipboard>,
    setter: UseStateSetter<String>,
) -> Html {
    html! {
        <button class="btn-clipboard" onclick={
            Callback::from(move |_| {
                let setter = setter.clone();
                let resolve_closure = Closure::wrap(Box::new(move |data: JsValue| {
                    if let Some(text) = data.as_string() {
                        setter.set(text);
                    }
                }) as Box<dyn FnMut(JsValue)>);

                let _ = clipboard.read_text().then(&resolve_closure);
                resolve_closure.forget();
            })
        }>
            { "粘贴" }
        </button>
    }
}
