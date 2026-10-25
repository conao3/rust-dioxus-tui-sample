use dioxus::prelude::*;

fn main() -> anyhow::Result<()> {
    dioxus_tui::launch_cfg(app, dioxus_tui::Config::new())
}

fn app() -> Element {
    let mut value = use_signal(|| String::from("hello world"));

    rsx! {
        div {
            p { "Input text: {value}" }
            input {
                value: "{value}",
                oninput: move |event| value.set(event.value()),
            }
        }
    }
}
