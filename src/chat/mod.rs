use dioxus::prelude::*;
use nultr_shared_lib::request::AuthUserData;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Chat() -> Element {
 //   let auth_user_data = use_context::<AuthUserData>;
//    let user_id = auth_user_data().user_id;

    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
//            "{user_id}"
            a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
            a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
            a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
            a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
