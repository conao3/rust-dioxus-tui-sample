use dioxus::prelude::*;

fn main() -> anyhow::Result<()> {
    dioxus_tui::launch(app)
}

fn app() -> Element {
    let names = ["jim", "bob", "jane", "doe", "a"];

    rsx! {
        ul {
            for name in names {
                li { key: "{name}", "{name}" }
            }
        }
    }
}
