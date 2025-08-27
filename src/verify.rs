use yew::prelude::*;

#[function_component]
pub fn Verify() -> Html {
    let ver_key_text = use_state(String::default);

    html! {
        <div class="my-4 col">
            <div class="col mb-3">
                <label
                    for="verKey"
                    class="form-label"
                >
                    { "公钥" }
                </label>
                <input
                    type="text"
                    class="form-control"
                    id="verKey"
                />
            </div>
            <div class="row">
                <div class="col-md-6 mb-3">
                    <label
                        for="textToVerify"
                        class="form-label"
                    >
                        { "需要验证的文本(不含签名)" }
                    </label>
                    <textarea
                        class="form-control"
                        id="textToVerify"
                        rows="3"
                    />
                </div>
                <div class="col-md-6 mb-3">
                    <label
                        for="signature"
                        class="form-label"
                    >
                        { "签名" }
                    </label>
                    <textarea
                        class="form-control"
                        id="signature"
                        rows="2"
                    />
                </div>
            </div>
        </div>
    }
}
