use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component]
pub fn Sign() -> Html {
    let text_to_sign_handle = use_state(String::default);
    let text_to_sign = (*text_to_sign_handle).clone();

    let on_text_input = {
        let text_to_sign_handle = text_to_sign_handle.clone();
        Callback::from(move |e: InputEvent| {
            let input = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            if let Some(input) = input {
                text_to_sign_handle.set(input.value());
            }
        })
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
                        value={text_to_sign.clone()}
                    />
                </div>
            </div>
        </div>
    }
}
