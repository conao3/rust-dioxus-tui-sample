use dioxus::prelude::*;

fn main() -> anyhow::Result<()> {
    dioxus_tui::launch(app)
}

fn app() -> Element {
    let val = 42;
    let name = if val == 42 { "Jack" } else { "Bob" };
    let some_user_name = Some("bob");

    rsx! {
        div {
            "hello world"
            "hello {val}"
            "hello {name}"
            if val == 42 {
                "Jack"
            }
            if let Some(name) = some_user_name {
                "hello {name}"
            }
        }
    }
}
