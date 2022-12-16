use dioxus::prelude::*;

fn main() {
    dioxus_tui::launch(app);
}

fn app(cx: Scope) -> Element {
    let val = 42;
    let name = if val == 42 { "Jack" } else { "Bob" };

    let some_user_name = Some("bob");

    cx.render(rsx! (
        div {
            "hello world"
            "hello {val}"
            "hello {name}"
            "hello "(val == 42).then(|| rsx!{
                "Jack"
            })
            some_user_name.map(|name| rsx!{
                "hello {name}"
            })
        }
    ))
}
