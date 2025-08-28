use base64ct::{Base64, Encoding};
use ed25519_dalek::{PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH, Verifier};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::utils::*;

#[function_component]
pub fn Verify() -> Html {
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
                    { "公钥" }
                </label>
                {
                    make_read_from_clipboard_btn(
                        clipboard.clone(),
                        ver_key_text.setter(),
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
                            ""
                        } else if ver_key_b.is_none() {
                            "格式错误，不是有效的Base64"
                        } else if (*ver_key_b).as_ref().map(|b| b.len()) != Some(PUBLIC_KEY_LENGTH) {
                            "长度错误，不是有效的Ed25519公钥"
                        } else if ver_key.map(|k| k.is_weak()) == Some(true) {
                            "弱公钥"
                        } else {
                            "公钥无效"
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
                        { "需要验证的文本(不含签名)" }
                    </label>
                    {
                        make_read_from_clipboard_btn(
                            clipboard.clone(),
                            text_to_verify.setter(),
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
                        { format!("长度: {} bytes", text_len) }
                    </div>
                </div>
                <div class="col-md-6 mb-3">
                    <label
                        for="signature"
                        class="form-label"
                    >
                        { "签名" }
                    </label>
                    {
                        make_read_from_clipboard_btn(
                            clipboard.clone(),
                            signature_text.setter(),
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
                            { "验证成功！这是签名者发布的内容" }
                        </div>
                    } else {
                        <div class="invalid-feedback">
                            {
                                if signature_text.is_empty() {
                                    ""
                                } else if signature_b.is_none() {
                                    "格式错误，不是有效的Base64"
                                } else if (*signature_b).as_ref().map(|b| b.len()) != Some(SIGNATURE_LENGTH) {
                                    "长度错误，不是有效的Ed25519签名"
                                } else if signature.is_none() {
                                    "签名无效"
                                } else {
                                    "验证失败！这很可能不是签名者发布的内容"
                                }
                            }
                        </div>
                    }
                </div>
            </div>
        </div>
    }
}
