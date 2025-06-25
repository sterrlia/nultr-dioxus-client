use dioxus::prelude::*;

use crate::state::{AUTH_STATE, AuthState};

#[derive(Props, PartialEq, Clone)]
pub struct AuthGuardProps {
    pub child: Element,
}

#[component]
pub fn AuthGuard(props: AuthGuardProps) -> Element {
    let nav = use_navigator();

    match &*AUTH_STATE.read() {
        AuthState::Unauthenticated => {
            nav.replace("/login");
            rsx! {}
        }
        AuthState::Authenticated(auth_user_data) => {
            rsx! {
                {props.child}
            }
        }
    }
}
