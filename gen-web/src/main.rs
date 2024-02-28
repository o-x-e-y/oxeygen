use leptos::*;
// use gen_core::libdof::dofinitions::KeyboardType;
// stylance::import_crate_style!(pub app_style, "css/app.css");

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Keyboard />
    }
}

pub struct IsoKeyboard<'a> {
    pub num_row: [&'a str; 13],
    pub top_row: [&'a str; 12],
    pub home_row: [&'a str; 12],
    pub bot_row: [&'a str; 11],
}

#[component]
pub fn Keyboard() -> impl IntoView {
    // match keyboard_type {
    //     KeyboardType::Iso => (),
    //     _ => ()
    // };

    view! {
        <h1>"Please pretend this is an iso keyboard :3"</h1>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
