use argon2::{Algorithm, Argon2, Params, Version};
use base64ct::{Base64, Encoding};
use ed25519::signature::Signer;
use ed25519_dalek::{SECRET_KEY_LENGTH, SigningKey};
use gloo_timers::future::sleep;
use std::time::Duration;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_hooks::use_clipboard;

use crate::utils::*;

fn derive_signing_key(password: &[u8], salt: &[u8]) -> SigningKey {
    let mut secret_key_bytes = [0u8; SECRET_KEY_LENGTH];
    Argon2::new_with_secret(
        b"_pepper_",
        Algorithm::default(),
        Version::default(),
        Params::new(
            Params::DEFAULT_M_COST,
            Params::DEFAULT_T_COST,
            Params::DEFAULT_P_COST,
            None,
        )
        .unwrap(),
    )
    .unwrap()
    .hash_password_into(password, salt, &mut secret_key_bytes)
    .unwrap();

    SigningKey::from_bytes(&secret_key_bytes)
}

#[function_component]
pub fn Sign() -> Html {
    let clipboard = use_clipboard();

    let username = use_state(String::default);
    let password = use_state(String::default);
    let text_to_sign = use_state(String::default);
    let signing_key = use_state(|| None);
    let is_calcing = use_state(|| false);

    let text_original = (*text_to_sign).clone();
    let text_normalized = remove_trailing_blank_lines(&normalize_newlines(&text_original));
    let text_after_sign = use_memo((text_normalized, (*signing_key).clone()), |(text, key)| {
        if text.is_empty() {
            String::default()
        } else {
            key.as_ref()
                .map(|key: &SigningKey| {
                    let text_bytes = text.as_bytes();
                    let len = text_bytes.len();
                    let signature = key.sign(text_bytes);
                    format!(
                        "{}\n\n长度: {} bytes\n签名: {}",
                        text,
                        len,
                        Base64::encode_string(&signature.to_bytes())
                    )
                })
                .unwrap_or_default()
        }
    });

    let on_calc_key = {
        let is_calcing = is_calcing.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            is_calcing.set(true);
        })
    };
    let on_text_input = {
        let text_to_sign = text_to_sign.clone();
        Callback::from(move |e: InputEvent| text_to_sign.set(textarea_value(e.target())))
    };

    let on_username_input = {
        let signing_key = signing_key.clone();
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            username.set(input_value(e.target()));
            signing_key.set(None);
        })
    };
    let on_password_input = {
        let signing_key = signing_key.clone();
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            password.set(input_value(e.target()));
            signing_key.set(None);
        })
    };

    {
        let signing_key = signing_key.clone();
        let is_calcing = is_calcing.clone();

        // salt only required uniqueness
        let b_salt = format!("easy_sign:{}:手持两把锟斤拷", *username).into_bytes();
        let b_password = (*password).clone().into_bytes();

        use_effect_with(*is_calcing, move |&is_c| {
            if is_c {
                spawn_local(async move {
                    // sleep 10ms to re-render component before blocking
                    sleep(Duration::from_millis(10)).await;

                    let key = derive_signing_key(&b_password, &b_salt);
                    signing_key.set(Some(key));

                    is_calcing.set(false);
                });
            }
        })
    }

    html! {
        <div class="my-4 col">
            // To trigger the browser to save the password
            <form class="row align-items-end mb-4" onsubmit={on_calc_key}>
                <div class="col-md-4 mb-2">
                    <div class="form-floating">
                        <input
                            id="username"
                            name="username"
                            type="text"
                            class="form-control"
                            placeholder="用户名"
                            aria-label="Name"
                            value={(*username).clone()}
                            oninput={on_username_input}
                        />
                        <label for="username">{ "用户名" }</label>
                    </div>
                </div>
                <div class="col-md-5 mb-2">
                    <div class="form-floating">
                        <input
                            id="password"
                            name="password"
                            type="password"
                            class="form-control"
                            placeholder="密码"
                            aria-label="Password"
                            value={(*password).clone()}
                            oninput={on_password_input}
                        />
                        <label for="password">{ "密码" }</label>
                    </div>
                </div>
                <div class="col-md-3 mb-2">
                    if !*is_calcing {
                        if signing_key.is_none() {
                            <button
                                class="btn btn-primary"
                                //onclick={on_calc_key}
                                disabled={
                                    username.is_empty() || password.is_empty()
                                }
                            >
                                { "计算私钥" }
                            </button>
                        } else {
                            <button
                                class="btn btn-warning"
                                type="button"
                            >
                                { "导出私钥" }
                            </button>
                        }
                    } else {
                        <span class="text-muted">{ "计算中..." }</span>
                    }
                </div>
            </form>
            <div class="row">
                <div class="col-md-6 mb-3">
                    <label
                        for="textToSign"
                        class="form-label"
                    >
                        { "需要签名的文本" }
                    </label>
                    { make_read_from_clipboard_btn(clipboard.clone(), text_to_sign.setter()) }
                    <textarea
                        class="form-control"
                        id="textToSign"
                        rows="3"
                        oninput={on_text_input}
                        value={text_original.clone()}
                    />
                </div>
                <div class="col-md-6 mb-3">
                    <label
                        for="withSignature"
                        class="form-label"
                    >
                        { "签名后文本" }
                        { make_write_to_clipboard_btn(clipboard.clone(), (*text_after_sign).clone()) }
                    </label>
                    <textarea
                        class="form-control"
                        id="withSignature"
                        readonly={true}
                        rows="3"
                        value={(*text_after_sign).clone()}
                    />
                </div>
            </div>
            if let Some(key) = signing_key.as_ref() {
                <div class="col mb-5">
                    <label
                        for="pubKey"
                        class="form-label"
                    >
                        { "公钥 (验证用)" }
                    </label>
                    { make_write_to_clipboard_btn(clipboard.clone(), Base64::encode_string(key.verifying_key().as_bytes())) }
                    <input
                        type="text"
                        class="form-control"
                        id="pubKey"
                        readonly={true}
                        value={
                             Base64::encode_string(key.verifying_key().as_bytes())
                        }
                    />
                </div>
            }
        </div>
    }
}
