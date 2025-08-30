use argon2::{Algorithm, Argon2, Params, Version};
use base64ct::{Base64, Encoding, LineEnding};
use ed25519_dalek::pkcs8::EncodePrivateKey;
use ed25519_dalek::{SECRET_KEY_LENGTH, Signer, SigningKey};
use gloo_file::Blob;
use gloo_timers::future::sleep;
use std::time::Duration;
use web_sys::{HtmlTextAreaElement, Url};
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::utils::*;

fn derive_signing_key(
    password: &[u8],
    salt: &[u8],
    m_cost: u32,
    t_cost: u32,
) -> Option<SigningKey> {
    let mut secret_key_bytes = [0u8; SECRET_KEY_LENGTH];
    Argon2::new_with_secret(
        env!("PEPPER").as_bytes(),
        Algorithm::default(),
        Version::default(),
        Params::new(m_cost, t_cost, Params::DEFAULT_P_COST, None).ok()?,
    )
    .ok()?
    .hash_password_into(password, salt, &mut secret_key_bytes)
    .ok()?;

    Some(SigningKey::from_bytes(&secret_key_bytes))
}

#[derive(Properties, PartialEq)]
pub struct SignProps {
    pub loc: String,
}

#[function_component]
pub fn Sign(_props: &SignProps) -> Html {
    let clipboard = use_memo((), |_| get_clipboard());

    let username = use_state(String::default);
    let password = use_state(String::default);
    let text_to_sign = use_state(String::default);
    let signing_key = use_state(|| None);
    let is_calcing = use_state(|| false);
    let key_blob_url = use_state(String::default);
    let m_cost = use_state(|| Params::DEFAULT_M_COST * 3);
    let t_cost = use_state(|| Params::DEFAULT_T_COST * 3);

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

    let textarea2_ref = use_node_ref();
    {
        let textarea_ref = textarea2_ref.clone();
        use_effect_with((*text_after_sign).clone(), move |_| {
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

    let on_m_cost_input = {
        let m_cost = m_cost.clone();
        Callback::from(move |e: Event| {
            let v = input_value(e.target())
                .parse::<u32>()
                .unwrap_or(Params::DEFAULT_M_COST);
            m_cost.set(v.max(Params::MIN_M_COST));
        })
    };

    let on_t_cost_input = {
        let t_cost = t_cost.clone();
        Callback::from(move |e: Event| {
            let v = input_value(e.target())
                .parse::<u32>()
                .unwrap_or(Params::DEFAULT_T_COST);
            t_cost.set(v.max(Params::MIN_T_COST));
        })
    };

    {
        let signing_key = signing_key.clone();
        let is_calcing = is_calcing.clone();
        let key_blob_url = key_blob_url.clone();

        let m_cost = *m_cost;
        let t_cost = *t_cost;

        // salt only required uniqueness
        let b_salt = format!("daily_sign:{}:手持两把锟斤拷", *username).into_bytes();
        let b_password = (*password).clone().into_bytes();

        use_effect_with(*is_calcing, move |&is_c| {
            if is_c {
                spawn_local(async move {
                    // sleep 10ms to re-render component before blocking
                    sleep(Duration::from_millis(10)).await;

                    let key = derive_signing_key(&b_password, &b_salt, m_cost, t_cost);

                    if let Some(key) = key.as_ref() {
                        let pem = key.to_pkcs8_pem(LineEnding::LF).unwrap_or_default();
                        let blob = Blob::new(pem.as_str());
                        key_blob_url.set(
                            Url::create_object_url_with_blob(blob.as_ref()).unwrap_or_default(),
                        );
                    }
                    signing_key.set(key);
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
                            placeholder={ t!("username") }
                            aria-label="Name"
                            value={(*username).clone()}
                            oninput={on_username_input}
                        />
                        <label for="username">{ t!("username") }</label>
                    </div>
                </div>
                <div class="col-md-5 mb-2">
                    <div class="form-floating">
                        <input
                            id="password"
                            name="password"
                            type="password"
                            class="form-control"
                            placeholder={ t!("password") }
                            aria-label="Password"
                            value={(*password).clone()}
                            oninput={on_password_input}
                        />
                        <label for="password">{ t!("password") }</label>
                    </div>
                </div>

                <div class="col-md-3 mb-2">
                    if !*is_calcing {
                        if signing_key.is_none() {
                            <div class="btn-group">
                                <button
                                    class="btn btn-primary"
                                    disabled={
                                        username.is_empty() || password.is_empty()
                                    }
                                >
                                    { "计算私钥" }
                                </button>
                                <button
                                    type="button"
                                    class="btn btn-primary dropdown-toggle dropdown-toggle-split"
                                    data-bs-toggle="dropdown"
                                    data-bs-auto-close="outside"
                                    aria-expanded="false"
                                    disabled={
                                        username.is_empty() || password.is_empty()
                                    }
                                >
                                    <span class="visually-hidden">{ "Toggle Dropdown" }</span>
                                </button>
                                <div class="col dropdown-menu px-2" style="width: max-content;">
                                    <div class="row mb-2">
                                        <label for="m_cost" class="col-3 col-form-label">{ "m_cost " }</label>
                                        <div class="col-9">
                                            <input
                                                type="number"
                                                class="form-control"
                                                id="m_cost"
                                                min={Params::MIN_M_COST.to_string()}
                                                max={Params::MAX_M_COST.to_string()}
                                                value={m_cost.to_string()}
                                                onchange={on_m_cost_input}
                                            />
                                        </div>
                                    </div>
                                    <div class="row mb-1">
                                        <label for="t_cost" class="col-3 col-form-label">{ "t_cost" }</label>
                                        <div class="col-9">
                                            <input
                                                type="number"
                                                class="form-control"
                                                id="t_cost"
                                                min={Params::MIN_T_COST.to_string()}
                                                max={Params::MAX_T_COST.to_string()}
                                                value={t_cost.to_string()}
                                                onchange={on_t_cost_input}
                                            />
                                        </div>
                                    </div>
                                </div>
                            </div>
                        } else {
                            <a
                                class="btn btn-danger"
                                href={(*key_blob_url).clone()}
                                download={ format!("{}_signing_key.pem", *username) }
                            >
                                { "⚠️ 导出私钥" }
                            </a>
                        }
                    } else {
                        <span class="text-muted">{ "计算中..." }</span>
                    }
                </div>
            </form>
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
            <div class="row">
                <div class="col-md-6 mb-3">
                    <label
                        for="textToSign"
                        class="form-label"
                    >
                        { "需要签名的文本" }
                    </label>
                    {
                        make_read_from_clipboard_btn(
                            clipboard.clone(),
                            text_to_sign.setter(),
                        )
                    }
                    <textarea
                        class="form-control"
                        id="textToSign"
                        ref={textarea1_ref}
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
                        ref={textarea2_ref}
                        readonly={true}
                        rows="3"
                        value={(*text_after_sign).clone()}
                    />
                </div>
            </div>
        </div>
    }
}
