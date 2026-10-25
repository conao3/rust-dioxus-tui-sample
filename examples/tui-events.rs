use dioxus::prelude::*;

fn main() -> anyhow::Result<()> {
    dioxus_tui::launch(app)
}

const MAX_EVENTS: usize = 8;

fn app() -> Element {
    let mut events = use_signal(Vec::<String>::new);
    let visible_events = events.read();
    let first_index = visible_events.len().saturating_sub(MAX_EVENTS);
    let mut log_event = move |event: String| {
        events.with_mut(|events| {
            events.push(event);
            if events.len() > MAX_EVENTS {
                events.remove(0);
            }
        });
    };

    rsx! {
        div {
            width: "100%",
            height: "100%",
            flex_direction: "column",
            div {
                width: "80%",
                height: "50%",
                border_width: "1px",
                justify_content: "center",
                align_items: "center",
                background_color: "hsl(248, 53%, 58%)",
                onmousemove: move |event| log_event(format!("MouseMove: {:?}", event.data())),
                onclick: move |event| log_event(format!("MouseClick: {:?}", event.data())),
                ondoubleclick: move |event| log_event(format!("MouseDoubleClick: {:?}", event.data())),
                onmousedown: move |event| log_event(format!("MouseDown: {:?}", event.data())),
                onmouseup: move |event| log_event(format!("MouseUp: {:?}", event.data())),
                onwheel: move |event| log_event(format!("Wheel: {:?}", event.data())),
                onkeydown: move |event| log_event(format!("KeyDown: {:?}", event.data())),
                onkeyup: move |event| log_event(format!("KeyUp: {:?}", event.data())),
                onkeypress: move |event| log_event(format!("KeyPress: {:?}", event.data())),
                onfocusin: move |event| log_event(format!("FocusIn: {:?}", event.data())),
                onfocusout: move |event| log_event(format!("FocusOut: {:?}", event.data())),
                "Hover, click, type or scroll to see the info down below"
            },
            div {
                width: "80%",
                height: "50%",
                flex_direction: "column",
                for event in visible_events[first_index..].iter() {
                    p { "{event}" }
                }
            },
        }
    }
}
