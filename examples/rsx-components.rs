#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props)]
struct TitleCardProps<'a> {
    title: &'a str,
}

fn TitleCard<'a>(cx: Scope<'a, TitleCardProps<'a>>) -> Element {
    cx.render(rsx!{
        h1 { "{cx.props.title}" }
    })
}

fn main() {
    dioxus_tui::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        TitleCard {
            title: "hello world"
        }
    ))
}
