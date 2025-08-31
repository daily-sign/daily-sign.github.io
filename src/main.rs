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
                    { t!("language") }
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

            <div class={classes!((!*is_sign).then_some("d-none"))}>
                <sign::Sign loc={(*locale).clone()}/>
            </div>
            <div class={classes!(is_sign.then_some("d-none"))}>
                <verify::Verify loc={(*locale).clone()} />
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
                <small class="text-muted">
                    <span class="me-4">{ "🄯2025 Daily Sign" }</span>
                    <a href="//github.com/daily-sign/daily-sign.github.io" target="_blank" rel="noopener noreferrer">
                        <img src="./static/github-mark.svg" alt="GitHub" height="18" />
                    </a>
                </small>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
