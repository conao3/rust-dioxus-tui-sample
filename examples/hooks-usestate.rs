use dioxus::prelude::*;
use dioxus_tui::prelude::*;
use dioxus_html::FormData;
use dioxus_tui::Config;

fn main() {
    dioxus_tui::launch_cfg(app, Config::new());
}

fn app(cx: Scope) -> Element {
    let value = use_state(cx, || String::from("hello world"));

    cx.render(rsx! {
        div {
            Input {
                value: "a"
            }
        }
    })
}
