use yew::prelude::*;

mod crypto;
mod sign;
mod utils;
mod verify;

#[function_component]
fn App() -> Html {
    let is_sign = use_state(|| true);

    let on_switch = {
        let is_sign = is_sign.clone();
        Callback::from(move |_| {
            is_sign.set(!*is_sign);
        })
    };

    html! {
        <div class="container">
            <h1 class="text-center my-5 font-a">{ "Easy Sign" }</h1>

            <ul class="nav nav-tabs">
                <li class="nav-item">
                    <a
                        class={classes!("nav-link", if *is_sign { "active" } else { "" })}
                        aria-current="page" href="#" onclick={on_switch.clone()}
                    >
                        { "签名" }
                    </a>
                </li>
                <li class="nav-item">
                    <a
                        class={classes!("nav-link", if !*is_sign { "active" } else { "" })}
                        aria-current="page" href="#" onclick={on_switch.clone()}
                    >
                        { "验证" }
                    </a>
                </li>
            </ul>

            if *is_sign {
                <sign::Sign />
            } else {
                <verify::Verify />
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
