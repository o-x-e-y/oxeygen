use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use stylance::{import_style, classes};
// use gen_core::libdof::dofinitions::KeyboardType;

import_style!(pub iso_keyboard, "../css/keyboard_iso.module.css");

#[derive(Debug, Clone)]
pub struct IsoKeyboard<'a> {
    pub num_row: [&'a str; 13],
    pub top_row: [&'a str; 12],
    pub home_row: [&'a str; 12],
    pub bot_row: [&'a str; 11],
}

#[component]
pub fn App() -> impl IntoView {
    let formatter = |text| format!("{text} - Oxeygen");
    provide_meta_context();

    view! {
        <Html lang="en"/>
        <Style id="main-sheet">{include_str!("../css/bundle.scss")}</Style>
        <Title formatter/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        <Link href="https://fonts.googleapis.com/css2?family=Roboto+Mono:wght@300&display=swap" rel="stylesheet"/>
        <Meta
            charset="UTF-8"
            name="description"
            content="Oxeygen is a blazingly fast keyboard analyzer written in Rust fire emoji rocket emoji"
        />
        <Router>
            <Routes>
                <Route path="" view=Home ssr=SsrMode::Async/>
            </Routes>
        </Router>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    static KEYBOARD_ISO: IsoKeyboard = IsoKeyboard {
        num_row: [
            "`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "[", "]",
        ],
        top_row: ["b", "c", "d", "l", "z", "j", "f", "o", "u", ",", "/", "="],
        home_row: ["n", "s", "t", "r", "v", "y", "h", "a", "e", "i", "-", "\\"],
        bot_row: ["q", "w", "g", "m", "x", "", "k", "p", "'", ";", "."],
    };

    provide_meta_context();

    view! {
        <Stylesheet id="main_sheet" href="../css/bundle.scss"/>
        <Keyboard keyboard=&KEYBOARD_ISO/>
    }
}

fn key_row(keys: &[&str]) -> View {
    keys
        .iter()
        .map(|k| view! { <div class=iso_keyboard::key>{k.to_string()}</div> })
        .collect_view()
}

#[component]
pub fn Keyboard<'a>(keyboard: &'a IsoKeyboard<'a>) -> impl IntoView {
    let space_row = key_row(&["CTL", "⌂", "ALT", "", "ALT", "⌂", "≡", "CTL"]);

    view! {
        <div class=iso_keyboard::keyboard_wrapper_iso>
            <div class=iso_keyboard::keyboard>
                <div class=classes!(iso_keyboard::row, iso_keyboard::num_row)>
                    {key_row(&keyboard.num_row)}
                    <div class=iso_keyboard::key>{"⇐"}</div>
                </div>
                <div class=classes!(iso_keyboard::row, iso_keyboard::top_row)>
                    <div class=classes!(iso_keyboard::key, iso_keyboard::key_left_align)>{"↹"}</div>
                    {key_row(&keyboard.top_row)}
                    <div class=classes!(iso_keyboard::key, iso_keyboard::return_top)>{"↵"}</div>
                </div>
                <div class=classes!(iso_keyboard::row, iso_keyboard::home_row)>
                    <div class=classes!(iso_keyboard::key, iso_keyboard::key_left_align)>{"⮸"}</div>
                    {key_row(&keyboard.home_row)}
                    <div class=classes!(iso_keyboard::key, iso_keyboard::return_bot)></div>
                </div>
                <div class=classes!(iso_keyboard::row, iso_keyboard::bot_row)>
                    <div class=classes!(iso_keyboard::key, iso_keyboard::key_left_align)>{"⇧"}</div>
                    {key_row(&keyboard.bot_row)}
                    <div class=iso_keyboard::key>{"⇧"}</div>
                </div>
                <div class=classes!(iso_keyboard::row, iso_keyboard::space_row)>
                    {space_row}
                </div>
            </div>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
