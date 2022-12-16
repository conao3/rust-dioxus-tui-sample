use dioxus::prelude::*;

fn main() {
    dioxus_tui::launch(app);
}

fn app(cx: Scope) -> Element {
    let names = ["jim", "bob", "jane", "doe", "a"];

    cx.render(rsx! (
        ul {
            names.iter().map(|name| rsx!{
                li { key: "{name}", "{name}" }
            })
        }
    ))
}
