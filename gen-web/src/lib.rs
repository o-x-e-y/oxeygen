#![allow(non_snake_case)]

mod keyboard_ansi;
mod keyboard_colstag;
mod keyboard_iso;
mod keyboard_ortho;

use dioxus::prelude::*;

use dioxus_router::{Link, Redirect, Route, Router};
use serde::Deserialize;

pub fn run() {
    // launch the web app
    dioxus_web::launch(App);
}

#[derive(Deserialize)]
struct ApiResponse {
    message: String,
    status: String,
}

static KEYBOARD_ISO: keyboard_iso::IsoKeyboard = keyboard_iso::IsoKeyboard {
    num_row: [
        "`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "[", "]",
    ],
    top_row: ["b", "c", "d", "l", "z", "j", "f", "o", "u", ",", "/", "="],
    home_row: ["n", "s", "t", "r", "v", "y", "h", "a", "e", "i", "-", "\\"],
    bot_row: ["q", "w", "g", "m", "x", "", "k", "p", "'", ";", "."],
};

static KEYBOARD_ANSI: keyboard_ansi::AnsiKeyboard = keyboard_ansi::AnsiKeyboard {
    num_row: [
        "`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "[", "]",
    ],
    top_row: [
        "b", "c", "d", "l", "z", "j", "f", "o", "u", ",", "/", "=", "\\",
    ],
    home_row: ["n", "s", "t", "r", "v", "y", "h", "a", "e", "i", "-"],
    bot_row: ["w", "g", "m", "x", "q", "k", "p", "'", ";", "."],
};

static KEYBOARD_ORTHO: keyboard_ortho::OrthoKeyboard = keyboard_ortho::OrthoKeyboard {
    num_row: ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"],
    top_row: ["b", "c", "d", "l", "z", "j", "f", "o", "u", ","],
    home_row: ["n", "s", "t", "r", "v", "y", "h", "a", "e", "i"],
    bot_row: ["q", "w", "g", "m", "x", "k", "p", "'", ";", "."],
};

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "anonymous" }
        link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Roboto+Mono:wght@300&display=swap" }
        style { include_str!("../css/app.css") }

        // keyboard_ansi::Keyboard { keyboard: &KEYBOARD_ANSI }
        keyboard_iso::Keyboard { keyboard: &KEYBOARD_ISO }
        // keyboard_ortho::Keyboard { keyboard: &KEYBOARD_ORTHO }
        // Router {
        //     ul {
        //         Link { to: "/",  li { "Go home!" } }
        //         Link { to: "/users",  li { "List all users" } }
        //         Link { to: "/blog", li { "Blog posts" } }

        //         Link { to: "/users/bill",  li { "List all users" } }
        //         Link { to: "/blog/5", li { "Blog post 5" } }
        //         Link { to: "/data/thing", li { "Show Thing data" } }
        //     }
        //     Route { to: "/", "Home" }
        //     Route { to: "/users", "User list" }
        //     Route { to: "/users/:name", User {} }
        //     Route { to: "/blog", "Blog list" }
        //     Route { to: "/blog/:post", BlogPost {} }
        //     Route { to: "/data/:name", Data {} }
        //     Route { to: "", "Err 404 Route Not Found" }
        // }
    })
}

fn Data(cx: Scope) -> Element {
    let name = dioxus_router::use_route(cx).last_segment().unwrap();
    let url = format!("data/{name}.json");

    let future = use_future(cx, &url, |url| async move {
        gloo_net::http::Request::new(&url)
            .send()
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });

    cx.render(match future.value() {
        Some(Ok(response)) => rsx! {
            button {
                onclick: move |_| future.restart(),
                "Click to fetch another doggo"
            }
            p { "message: {response.message}" }
            p { "status: {response.status}" }
        },
        Some(Err(_)) => rsx! { div { "Loading data failed" } },
        None => rsx! { div { "Loading data..." } },
    })
}

fn BlogPost(cx: Scope) -> Element {
    let post = dioxus_router::use_route(cx).last_segment().unwrap();

    cx.render(rsx! {
        div {
            h1 { "Reading blog post: {post}" }
            p { "example blog post" }
        }
    })
}

#[derive(Deserialize)]
struct Query {
    bold: bool,
}

fn User(cx: Scope) -> Element {
    let post = dioxus_router::use_route(cx).last_segment().unwrap();

    let query = dioxus_router::use_route(cx)
        .query::<Query>()
        .unwrap_or(Query { bold: false });

    cx.render(rsx! {
        div {
            h1 { "Reading blog post: {post}" }
            p { "example blog post" }

            if query.bold {
                rsx!{ b { "bold" } }
            } else {
                rsx!{ i { "italic" } }
            }
        }
    })
}
