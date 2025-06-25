mod auth;
mod chat;
mod context;
mod error_list;
mod event;
mod login_form;
mod state;

use chat::Chat;
use context::init_context;
use dioxus::prelude::*;
use error_list::ErrorList;
use login_form::LoginForm;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/login")]
    LoginForm,
    #[route("/chat")]
    Chat
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    init_context();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            style: "position: absolute; top: -50px; left: 0;",
            ErrorList {}
        }
        div {
            Router::<Route> {}
        }
    }
}
