#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
struct TitleCardProps {
    title: String,
}

fn TitleCard(props: TitleCardProps) -> Element {
    rsx! {
        h1 { "{props.title}" }
    }
}

fn main() -> anyhow::Result<()> {
    dioxus_tui::launch(app)
}

fn app() -> Element {
    rsx! {
        TitleCard { title: "hello world" }
    }
}
