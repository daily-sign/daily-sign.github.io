use base64ct::{Base64, Encoding};
use ed25519::signature::Signer;
use ed25519_dalek::SigningKey;
use yew::prelude::*;

use crate::crypto::derive_signing_key;
use crate::utils::{normalize_newlines, remove_trailing_blank_lines, textarea_value};

#[function_component]
pub fn Sign() -> Html {
    let text_to_sign_handle = use_state(String::default);
    let signing_key_handle = use_state(|| None);

    let text_to_sign = (*text_to_sign_handle).clone();
    let text_normalized = remove_trailing_blank_lines(&normalize_newlines(&text_to_sign));
    let text_after_sign = use_memo(
        (text_normalized, (*signing_key_handle).clone()),
        |(text, key)| {
            key.as_ref()
                .map(|key: &SigningKey| {
                    let text_bytes = text.as_bytes();
                    let len = text_bytes.len();
                    let signature = key.sign(text_bytes);
                    format!(
                        "{}\n\n长度: {}bytes\n签名: {}",
                        text,
                        len,
                        Base64::encode_string(&signature.to_bytes())
                    )
                })
                .unwrap_or_default()
        },
    );

    let on_calc_key = {
        let signing_key_handle = signing_key_handle.clone();
        Callback::from(move |_| {
            let signing_key = derive_signing_key();
            signing_key_handle.set(Some(signing_key));
        })
    };
    let on_text_input = {
        let text_to_sign_handle = text_to_sign_handle.clone();
        Callback::from(move |e: InputEvent| text_to_sign_handle.set(textarea_value(e)))
    };

    html! {
        <div class="my-4 col">
            <div class="row mb-4">
                <div class="col-md-4 mb-2">
                    <div class="form-floating">
                    <input
                        id="username"
                        type="text"
                        class="form-control"
                        placeholder="用户名"
                        aria-label="Name"
                    />
                    <label for="username">{ "用户名" }</label>
                </div>
                </div>
                <div class="col-md-5 mb-2">
                    <div class="form-floating" >
                    <input
                        id="password"
                        type="password"
                        class="form-control"
                        placeholder="密码"
                        aria-label="Password"
                    />
                    <label for="password">{ "密码" }</label>
                </div>
                </div>
                <div class="col-md-3 mb-2">
                    <button
                        class="btn btn-lg btn-primary"
                        onclick={on_calc_key}
                    >
                        { "计算私钥" }
                    </button>
                </div>
            </div>
            <div class="row">
                <div class="col-md-6 mb-3">
                    <label
                        for="textToSign"
                        class="form-label"
                    >
                        { "需要签名的文本" }
                    </label>
                    <textarea
                        class="form-control"
                        id="textToSign"
                        rows="3"
                        oninput={on_text_input}
                        value={text_to_sign.clone()}
                    >
                    </textarea>
                </div>
                <div class="col-md-6 mb-3">
                    <label
                        for="signature"
                        class="form-label"
                    >
                        { "签名后文本" }
                    </label>
                    <textarea
                        class="form-control"
                        id="signature"
                        readonly={true}
                        rows="3"
                        value={(*text_after_sign).clone()}
                    />
                </div>
            </div>
        </div>
    }
}
