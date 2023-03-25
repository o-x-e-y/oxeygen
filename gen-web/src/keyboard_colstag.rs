use dioxus::prelude::*;

pub struct ColstagKeyboard<'a> {
    pub top_row: [&'a str; 10],
    pub home_row: [&'a str; 10],
    pub bot_row: [&'a str; 10],
}

#[inline_props]
pub fn Keyboard<'a>(cx: Scope, keyboard: &'a ColstagKeyboard<'a>) -> Element {
    cx.render(rsx! {
        style { include_str!("../css/keyboard_colstag.css") }
        div { id: "keyboard-wrapper-colstag",
            div { id: "keyboard",
                div { class: "row top-row",
                    div { class: "key key-left-align", "↹" }
                    keyboard.top_row
                        .into_iter()
                        .map(|k| rsx!{ div { class: "key", k } })
                    div { class: "key return-top", "↵" }
                }
                div { class: "row home-row",
                    div { class: "key key-left-align", "⮸" }
                    keyboard.home_row
                        .into_iter()
                        .map(|k| rsx!{ div { class: "key", k } })
                    div { class: "key return-bot" }
                }
                div { class: "row bot-row",
                    div { class: "key key-left-align", "⇧" }
                    keyboard.bot_row
                        .into_iter()
                        .map(|k| rsx!{ div { class: "key", k } })
                    div { class: "key", "⇧" }
                }
                div { class: "row space-row",
                    div { class: "key key-left-align", "CTL" }
                    ["⌂", "ALT", "", "ALT", "⌂", "≡", "CTL"]
                        .into_iter()
                        .map(|k| rsx!{ div { class: "key", k } })
                }
            }
        }
    })
}
