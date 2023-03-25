use dioxus::prelude::*;

pub struct OrthoKeyboard<'a> {
    pub num_row: [&'a str; 10],
    pub top_row: [&'a str; 10],
    pub home_row: [&'a str; 10],
    pub bot_row: [&'a str; 10],
}

#[inline_props]
pub fn Keyboard<'a>(cx: Scope, keyboard: &'a OrthoKeyboard<'a>) -> Element {
    cx.render(rsx! {
        style { include_str!("../css/keyboard_ortho.css") }
        div { id: "keyboard-wrapper-ortho",
            div { id: "keyboard",
                div { class: "row num-row-ortho",
                    div { class: "key", "`" }
                    keyboard.num_row
                        .into_iter()
                        .map(|k| rsx!{ div { class: "key", k } })
                    div { class: "key", "⇐" }
                }
                div { class: "row top-row-ortho",
                    div { class: "key", "↹" }
                    keyboard.top_row
                        .into_iter()
                        .map(|k| rsx!{ div { class: "key", k } })
                    div { class: "key", "↵" }
                }
                div { class: "row home-row-ortho",
                    div { class: "key", "⮸" }
                    keyboard.home_row
                        .into_iter()
                        .map(|k| rsx!{ div { class: "key", k } })
                    div { class: "key", "/" }
                }
                div { class: "row bot-row-ortho",
                    div { class: "key", "⇧" }
                    keyboard.bot_row
                        .into_iter()
                        .map(|k| rsx!{ div { class: "key", k } })
                    div { class: "key", "⇧" }
                }
                div { class: "row space-row-ortho",
                    div { class: "key key-left-align", "CTL" }
                    ["⌂", "ALT", "", "ALT", "⌂", "≡", "CTL"]
                        .into_iter()
                        .map(|k| rsx!{ div { class: "key", k } })
                }
            }
        }
    })
}
