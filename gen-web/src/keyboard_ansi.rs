use dioxus::prelude::*;

pub struct AnsiKeyboard<'a> {
    pub num_row: [&'a str; 13],
    pub top_row: [&'a str; 13],
    pub home_row: [&'a str; 11],
    pub bot_row: [&'a str; 10],
}

#[inline_props]
pub fn Keyboard<'a>(cx: Scope, keyboard: &'a AnsiKeyboard<'a>) -> Element {
    cx.render(rsx! {
        style { include_str!("../css/keyboard_ansi.css") }
        div { id: "keyboard-wrapper-ansi",
            div { id: "keyboard",
                div { class: "row num-row-ansi",
                    keyboard.num_row
                        .into_iter()
                        .map(|k| rsx!{ div { class: "key", k } })
                    div { class: "key", "⇐" }
                }
                div { class: "row top-row-ansi",
                    div { class: "key key-left-align", "↹" }
                    keyboard.top_row
                        .into_iter()
                        .map(|k| rsx!{ div { class: "key", k } })
                }
                div { class: "row home-row-ansi",
                    div { class: "key key-left-align", "⮸" }
                    keyboard.home_row
                        .into_iter()
                        .map(|k| rsx!{ div { class: "key", k } })
                        div { class: "key", "↵" }
                }
                div { class: "row bot-row-ansi",
                    div { class: "key key-left-align", "⇧" }
                    keyboard.bot_row
                        .into_iter()
                        .map(|k| rsx!{ div { class: "key", k } })
                    div { class: "key", "⇧" }
                }
                div { class: "row space-row-ansi",
                    div { class: "key key-left-align", "CTL" }
                    ["⌂", "ALT", "", "ALT", "⌂", "≡", "CTL"]
                        .into_iter()
                        .map(|k| rsx!{ div { class: "key", k } })
                }
            }
        }
    })
}
