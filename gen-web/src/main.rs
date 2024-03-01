mod keyboard;

use gen_core::libdof::Dof;
use keyboard::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
// use gen_core::libdof::dofinitions::KeyboardType;

#[derive(Debug, Clone)]
pub struct IsoKeyboardStruct<'a> {
    pub num_row: [&'a str; 13],
    pub top_row: [&'a str; 12],
    pub home_row: [&'a str; 12],
    pub bot_row: [&'a str; 11],
}

#[component]
pub fn App() -> impl IntoView {
    console_error_panic_hook::set_once();

    provide_meta_context();

    let formatter = |text| format!("{text} - Oxeygen");

    view! {
        <Html lang="en"/>
        <Title formatter/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=Roboto+Mono:wght@300&display=swap"
            rel="stylesheet"
        />

        <Meta
            charset="UTF-8"
            name="description"
            content="Oxeygen is a blazingly fast keyboard analyzer written in Rust fire emoji rocket emoji"
        />
        // <Router>
        //     <Routes>
        //         <Route path="" view=Home/>
        //         <Route path="garf" view=Thing/>
        //     </Routes>
        // </Router>
        <Home/>
    }
}

#[component]
pub fn Thing() -> impl IntoView {
    view! {
        <div style="margin-top: 40px; padding: 1cqw; background: linear-gradient(90deg, rgba(34,34,34,1) 0%, rgba(41,41,41,1) 35%, rgba(41,41,41,1) 65%, rgba(34,34,34,1) 100%); text-align: center">
            <h1>{"Garfsmie"}</h1>
        </div>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    static KEYBOARD_ISO: IsoKeyboardStruct = IsoKeyboardStruct {
        num_row: [
            "`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "[", "]",
        ],
        top_row: ["b", "c", "d", "l", "z", "j", "f", "o", "u", ",", "/", "="],
        home_row: ["n", "s", "t", "r", "v", "y", "h", "a", "e", "i", "-", "\\"],
        bot_row: ["q", "w", "g", "m", "x", "", "k", "p", "'", ";", "."],
    };

    let _qwerty =
        serde_json::from_str::<Dof>(include_str!("../public/dofs/minimal_valid.dof")).unwrap();

    provide_meta_context();

    view! { <IsoKeyboard keyboard=&KEYBOARD_ISO/> }
}

fn main() {
    leptos::mount_to_body(App)
}
