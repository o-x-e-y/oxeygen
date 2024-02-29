// use gen_core::libdof::{dofinitions::{Key, KeyboardType}, Dof};
use leptos::*;
use stylance::classes;

stylance::import_style!(pub iso, "../css/keyboard_iso.module.css");
// stylance::import_style!(pub ansi, "../css/keyboard_ansi.module.css");
// stylance::import_style!(pub ortho, "../css/keyboard_ortho.module.css");
// stylance::import_style!(pub colstag, "../css/keyboard_colstag.module.css");

// #[component]
// pub fn Keyboard(keyboard: Dof) -> impl IntoView {
//     match keyboard.board() {
//         KeyboardType::Ansi => view!{<AnsiKeyboard keyboard/>},
//         KeyboardType::Iso => todo!(),
//         KeyboardType::Ortho => todo!(),
//         KeyboardType::Colstag => todo!(),
//         KeyboardType::Custom(_) => todo!(),
//     }
// }

// #[component]
// fn AnsiKeyboard(keyboard: Dof) -> impl IntoView {
//     fn key_row(keys: &[Key]) -> View {
//         keys
//             .iter()
//             .map(|k| view! { <div class=ansi::key>{k.to_string()}</div> })
//             .collect_view()
//     }

//     let rows = keyboard.main_layer().inner();

//     view!{
//         <div class=ansi::keyboard_wrapper>
//             <div class=ansi::keyboard>
//                 <div class=classes!(ansi::row, ansi::num_row)>
//                     {key_row(&rows[0])}
//                 </div>
//                 <div class=classes!(ansi::row, ansi::top_row)>
//                     {key_row(&rows[1])}
//                 </div>
//                 <div class=classes!(ansi::row, ansi::home_row)>
//                     {key_row(&rows[2])}
//                 </div>
//                 <div class=classes!(ansi::row, ansi::bot_row)>
//                     {key_row(&rows[3])}
//                 </div>
//                 <div class=classes!(ansi::row, ansi::space_row)>
//                     {key_row(&rows[4])}
//                 </div>
//             </div>
//         </div>
//     }
// }

use crate::IsoKeyboardStruct;


#[component]
pub fn IsoKeyboard<'a>(keyboard: &'a IsoKeyboardStruct<'a>) -> impl IntoView {
    fn key_row(keys: &[&str]) -> View {
        keys
            .iter()
            .map(|k| view! { <div class=iso::key>{k.to_string()}</div> })
            .collect_view()
    }


    let space_row = key_row(&["CTL", "⌂", "ALT", "", "ALT", "⌂", "≡", "CTL"]);

    view! {
        <div class=iso::keyboard_wrapper>
            <div class=iso::keyboard>
                <div class=classes!(iso::row, iso::num_row)>
                    {key_row(&keyboard.num_row)}
                    <div class=iso::key>{"⇐"}</div>
                </div>
                <div class=classes!(iso::row, iso::top_row)>
                    <div class=classes!(iso::key, iso::key_left_align)>{"↹"}</div>
                    {key_row(&keyboard.top_row)}
                    <div class=classes!(iso::key, iso::return_top)>{"↵"}</div>
                </div>
                <div class=classes!(iso::row, iso::home_row)>
                    <div class=classes!(iso::key, iso::key_left_align)>{"⮸"}</div>
                    {key_row(&keyboard.home_row)}
                    <div class=classes!(iso::key, iso::return_bot)></div>
                </div>
                <div class=classes!(iso::row, iso::bot_row)>
                    <div class=classes!(iso::key, iso::key_left_align)>{"⇧"}</div>
                    {key_row(&keyboard.bot_row)}
                    <div class=iso::key>{"⇧"}</div>
                </div>
                <div class=classes!(iso::row, iso::space_row)>
                    {space_row}
                </div>
            </div>
        </div>
    }
}