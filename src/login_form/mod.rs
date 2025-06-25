use dioxus::prelude::*;
use nultr_shared_lib::request::{AuthUserData, LoginRequest, LoginResponse};
use rust_api_kit::http::client::BasicHttpClientTrait;

use crate::{context::Services, error_list::DisplayError, event::send_event, state::{AuthState, AUTH_STATE}};

#[derive(PartialEq, Props, Clone)]
struct LoginFormProps {}

#[component]
pub fn LoginForm() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    let on_submit = move |_| {
        spawn(async move {
            if let Err(event) = login(username(), password()).await {
                send_event(event)
            }
        });
    };

    rsx! {
        div {
            style: "max-width: 300px; margin: auto; padding-top: 100px;",
            h2 { "Login" }
            input {
                r#type: "text",
                placeholder: "Username",
                value: "{username}",
                oninput: move |e| username.set(e.value()),
            }
            input {
                r#type: "password",
                placeholder: "Password",
                value: "{password}",
                oninput: move |e| password.set(e.value()),
            }
            button {
                onclick: on_submit,
                "Login"
            }
        }
    }
}

async fn login(username: String, password: String) -> Result<(), DisplayError> {
    let request = LoginRequest { username, password };
    let response = use_context::<Services>().http_client.request(request).await??;

    *AUTH_STATE.write() = AuthState::Authenticated(AuthUserData {
        user_id: response.user_id,
        token: response.token
    });

    Ok(())
}
