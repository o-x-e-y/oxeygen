use dioxus::prelude::*;

pub struct IsoKeyboard<'a> {
    pub num_row: [&'a str; 13],
    pub top_row: [&'a str; 12],
    pub home_row: [&'a str; 12],
    pub bot_row: [&'a str; 11],
}

#[inline_props]
pub fn Keyboard<'a>(cx: Scope, keyboard: &'a IsoKeyboard<'a>) -> Element {
    cx.render(rsx! {
        style { include_str!("../css/keyboard_iso.css") }
        div { class: "keyboard-wrapper-iso",
            div { class: "keyboard",
                div { class: "row num-row",
                    keyboard.num_row
                        .into_iter()
                        .map(|k| rsx!{ div { class: "key", k } })
                    div { class: "key", "⇐" }
                }
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
                    div { class: "key", "CTL" }
                    ["⌂", "ALT", "", "ALT", "⌂", "≡", "CTL"]
                        .into_iter()
                        .map(|k| rsx!{ div { class: "key", k } })
                }
            }
        }
    })
}
