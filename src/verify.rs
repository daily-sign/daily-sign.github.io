use std::rc::Rc;

use base64ct::{Base64, Encoding};
use ed25519_dalek::{PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH, Verifier};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::utils::*;

#[derive(Properties, PartialEq)]
pub struct VerifyProps {
    pub loc: String,
}

#[function_component]
pub fn Verify(_props: &VerifyProps) -> Html {
    let clipboard = use_memo((), |_| get_clipboard());

    let ver_key_text = use_state(String::default);
    let text_to_verify = use_state(String::default);
    let signature_text = use_state(String::default);

    let text_original = (*text_to_verify).clone();
    let text_normalized = remove_trailing_blank_lines(&normalize_newlines(&text_original));
    let text_len = text_normalized.len();

    let ver_key_b = use_memo((*ver_key_text).clone(), |key_text| {
        (!key_text.is_empty())
            .then(|| Base64::decode_vec(key_text).ok())
            .flatten()
    });

    let ver_key = use_memo((*ver_key_b).clone(), |key_b| {
        key_b
            .as_ref()
            .and_then(|b| b.clone().try_into().ok())
            .and_then(|b| ed25519_dalek::VerifyingKey::from_bytes(&b).ok())
    });

    let signature_b = use_memo((*signature_text).clone(), |sig_text| {
        (!sig_text.is_empty())
            .then(|| Base64::decode_vec(sig_text).ok())
            .flatten()
    });

    let signature = use_memo((*signature_b).clone(), |sig_b| {
        sig_b
            .as_ref()
            .and_then(|b| b.clone().try_into().ok())
            .map(|b| ed25519_dalek::Signature::from_bytes(&b))
    });

    let verify_success = use_memo(
        (*ver_key, *signature, text_normalized.clone()),
        |(key, sig, text)| {
            if let (Some(key), Some(sig)) = (key, sig) {
                key.verify(text.as_bytes(), sig).is_ok()
            } else {
                false
            }
        },
    );

    let on_ver_key_input = {
        let ver_key_text = ver_key_text.clone();
        Callback::from(move |e: InputEvent| ver_key_text.set(input_value(e.target())))
    };

    let on_text_input = {
        let text_to_verify = text_to_verify.clone();
        Callback::from(move |e: InputEvent| text_to_verify.set(textarea_value(e.target())))
    };

    let on_signature_input = {
        let signature = signature_text.clone();
        Callback::from(move |e: InputEvent| signature.set(textarea_value(e.target())))
    };

    let textarea1_ref = use_node_ref();
    {
        let textarea_ref = textarea1_ref.clone();
        use_effect_with(text_original.clone(), move |_| {
            if let Some(textarea) = textarea_ref.cast::<HtmlTextAreaElement>() {
                textarea.style().set_property("height", "auto").unwrap();
                let height = textarea.scroll_height();
                textarea
                    .style()
                    .set_property("height", &format!("{}px", 100.max(height)))
                    .unwrap();
            }
        });
    }

    html! {
        <div class="my-4 col has-validation">
            <div class="col mb-3">
                <label
                    for="verKey"
                    class="form-label"
                >
                    { t!("public_key") }
                </label>
                {
                    make_read_from_clipboard_btn(
                        clipboard.clone(),
                        Rc::new(make_paste_cb(ver_key_text.setter())),
                    )
                }
                <input
                    type="text"
                    class={classes!(
                        "form-control",
                        if ver_key.map(|k| k.is_weak()) == Some(false) {
                            "is-valid"
                        } else {
                            "is-invalid"
                        }
                    )}
                    id="verKey"
                    value={(*ver_key_text).clone()}
                    oninput={on_ver_key_input}
                />
                <div class="invalid-feedback">
                    {
                        if ver_key_text.is_empty() {
                            t!("empty")
                        } else if ver_key_b.is_none() {
                            t!("invalid_format_base64")
                        } else if (*ver_key_b).as_ref().map(|b| b.len()) != Some(PUBLIC_KEY_LENGTH) {
                            t!("invalid_length_public_key")
                        } else if ver_key.map(|k| k.is_weak()) == Some(true) {
                            t!("weak_public_key")
                        } else {
                            t!("invalid_public_key")
                        }
                    }
                </div>
            </div>
            <div class="row">
                <div class="col-md-6 mb-3">
                    <label
                        for="textToVerify"
                        class="form-label"
                    >
                        { t!("text_to_verify") }
                    </label>
                    {
                        make_read_from_clipboard_btn(
                            clipboard.clone(),
                            Rc::new(make_paste_cb(text_to_verify.setter())),
                        )
                    }
                    <textarea
                        class="form-control"
                        id="textToVerify"
                        ref={textarea1_ref}
                        oninput={on_text_input}
                        value={(*text_to_verify).clone()}
                    />
                    <div id="passwordHelpBlock" class="form-text">
                        { t!("info.length", "length" => text_len) }
                    </div>
                </div>
                <div class="col-md-6 mb-3">
                    <label
                        for="signature"
                        class="form-label"
                    >
                        { t!("signature") }
                    </label>
                    {
                        make_read_from_clipboard_btn(
                            clipboard.clone(),
                            Rc::new(make_paste_cb(signature_text.setter())),
                        )
                    }
                    <textarea
                        class={classes!("form-control", if *verify_success { "is-valid" } else { "is-invalid" })}
                        id="signature"
                        rows="3"
                        oninput={on_signature_input}
                        value={(*signature_text).clone()}
                    />
                    if *verify_success {
                        <div class="valid-feedback">
                            { t!("verify_success") }
                        </div>
                    } else {
                        <div class="invalid-feedback">
                            {
                                if signature_text.is_empty() {
                                    t!("empty")
                                } else if signature_b.is_none() {
                                    t!("invalid_format_base64")
                                } else if (*signature_b).as_ref().map(|b| b.len()) != Some(SIGNATURE_LENGTH) {
                                    t!("invalid_length_signature")
                                } else if signature.is_none() {
                                    t!("invalid_signature")
                                } else {
                                    t!("verify_failed")
                                }
                            }
                        </div>
                    }
                </div>
            </div>
        </div>
    }
}
