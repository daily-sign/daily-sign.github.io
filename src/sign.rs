use base64ct::{Base64, Encoding};
use ed25519::signature::Signer;
use ed25519_dalek::SigningKey;
use gloo_timers::future::sleep;
use std::time::Duration;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::crypto::derive_signing_key;
use crate::sign;
use crate::utils::{input_value, normalize_newlines, remove_trailing_blank_lines, textarea_value};

#[function_component]
pub fn Sign() -> Html {
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
                        "{}\n\n长度: {}bytes\n签名: {}",
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
        Callback::from(move |_| is_calcing.set(true))
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
            <div class="row align-items-end mb-4">
                <div class="col-md-4 mb-2">
                    <div class="form-floating">
                    <input
                        id="username"
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
                    <div class="form-floating" >
                    <input
                        id="password"
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
                                class="btn btn-lg btn-primary"
                                onclick={on_calc_key}
                                disabled={
                                    username.is_empty() || password.is_empty()
                                }
                            >
                                { "计算私钥" }
                            </button>
                        } else {
                            <button
                                class="btn btn-lg btn-warning"
                            >
                                { "导出私钥" }
                            </button>
                        }
                    } else {
                        <span class="text-muted fs-5">{ "计算中..." }</span>
                    }
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
                        value={text_original.clone()}
                    />
                </div>
                <div class="col-md-6 mb-3">
                    <label
                        for="withSignature"
                        class="form-label"
                    >
                        { "签名后文本" }
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
                <div class="col mb-3">
                    <label
                        for="pubKey"
                        class="form-label"
                    >
                        { "公钥 (验证用)" }
                    </label>
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
