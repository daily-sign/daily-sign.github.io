use form_urlencoded;
use gloo_utils::window;
use yew::prelude::*;

mod sign;
mod utils;
mod verify;

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

#[function_component]
fn App() -> Html {
    let is_sign = use_state(|| window().location().hash().unwrap_or_default() != "#verify");
    let locale = use_state(|| {
        let locale = window().navigator().language().unwrap_or_default();
        rust_i18n::set_locale(&locale);
        locale
    });

    let on_switch = {
        let is_sign = is_sign.clone();
        Callback::from(move |_| {
            is_sign.set(!*is_sign);
        })
    };

    html! {
        <div class="min-vh-100 position-relative">
        <div class="container py-5">
            <h1 class="text-center mb-5 font-a">{ "Daily Sign" }</h1>

            // set locale dropdown btn
            <div class="dropdown lang-selector">
                <button class="btn btn-sm btn-outline-dark dropdown-toggle" type="button" id="langDropdownButton" data-bs-toggle="dropdown" aria-expanded="false">
                    <span class="ms-1 d-none d-sm-inline-block">{ t!("language") }</span>
                </button>
                <ul class="dropdown-menu" aria-labelledby="langDropdownButton">
                    {
                        ["zh-CN", "en"].iter().map(|loc| html!{
                            <li key={loc.to_string()}>
                                <a
                                    class="dropdown-item"
                                    href="#"
                                    onclick={{
                                        let locale = locale.clone();
                                        Callback::from(move |_| {
                                            rust_i18n::set_locale(loc);
                                            locale.set(loc.to_string());
                                        })
                                    }}
                                >
                                    { t!("language", locale = loc) }
                                </a>
                            </li>
                        }).collect::<Html>()
                    }
                </ul>
            </div>

            <ul class="nav nav-tabs">
                <li class="nav-item">
                    <a
                        class={classes!("nav-link", is_sign.then_some("active"))}
                        aria-current="page" href="#sign" onclick={on_switch.clone()}
                    >
                        { t!("title.sign") }
                    </a>
                </li>
                <li class="nav-item">
                    <a
                        class={classes!("nav-link", (!*is_sign).then_some("active"))}
                        aria-current="page" href="#verify" onclick={on_switch.clone()}
                    >
                        { t!("title.verify") }
                    </a>
                </li>
            </ul>

            <div class="pt-3 pb-1">
                <div class={classes!((!*is_sign).then_some("d-none"))}>
                    <sign::Sign loc={(*locale).clone()}/>
                </div>
                <div class={classes!(is_sign.then_some("d-none"))}>
                    <verify::Verify loc={(*locale).clone()} />
                </div>
            </div>

            <div class="text-body-secondary text-center mb-4">
                <small class="fst-italic">
                    { t!("tips.works_when_offline") }
                </small>
            </div>

            <div class="toast position-absolute top-0 left-50" role="alert" aria-live="polite" aria-atomic="true" data-bs-delay="3000" id="copyToast">
                <div class="d-flex">
                    <div class="toast-body">
                        { t!("copy_success") }
                    </div>
                    <button type="button" class="btn-close me-2 m-auto" data-bs-dismiss="toast" aria-label="Close"></button>
                </div>
            </div>

            </div>
            // a footer for copyleft and github log and link
            <div class="position-absolute bottom-0 bg-body-tertiary w-100 text-center py-2">
                <small class="text-body-secondary">
                    <span class="me-2">{ "🄯2025" }</span>
                    <a href="//github.com/daily-sign/daily-sign.github.io" target="_blank" rel="noopener noreferrer">
                        <img
                            src="https://github.com/daily-sign/daily-sign.github.io/actions/workflows/deploy.yml/badge.svg"
                            alt="GitHub Deploy"
                            class="align-text-bottom"
                        />
                    </a>
                    <span class="me-2"></span>
                    <a href="https://hitscounter.dev/history?url=daily-sign" target="_blank" rel="noopener noreferrer">
                        <img
                            src={
                                form_urlencoded::Serializer::new("https://hitscounter.dev/api/hit?tz=Asia%2FShanghai".to_owned())
                                    .append_pair("url", "daily-sign")
                                    .append_pair("label", &t!("visit_counter"))
                                    .append_pair("style", "flat")
                                    .append_pair("color", "#307efd")
                                    .append_pair("icon", "heart-fill")
                                    .finish()
                            }
                            class="align-text-bottom"
                            alt="Visit Counter"
                         />
                    </a>
                </small>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
