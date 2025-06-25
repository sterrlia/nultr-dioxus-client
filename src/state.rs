use dioxus::prelude::*;
use nultr_shared_lib::request::AuthUserData;

pub enum AuthState {
    Unauthenticated,
    Authenticated(AuthUserData)
}

pub static AUTH_STATE: GlobalSignal<AuthState> = Global::new(|| AuthState::Unauthenticated);
