use crate::event_listener;
use dioxus::prelude::*;
use nultr_client_lib::{define_error_event_enum, errors::IntoErrorMessage};
use nultr_shared_lib::request::{AuthenticatedUnexpectedErrorResponse, LoginErrorResponse, UnexpectedErrorResponse};
use rust_api_kit::http::client::UnexpectedHttpError;


define_error_event_enum! {
    DisplayError {
        String(String),
        AuthenticatedUnexpectedHttp(UnexpectedHttpError<AuthenticatedUnexpectedErrorResponse>),
        UnauthenticatedUnexpectedHttp(UnexpectedHttpError<UnexpectedErrorResponse>),
        Login(LoginErrorResponse)
    }
}


#[component]
pub fn ErrorList() -> Element {
    let mut errors = use_signal(|| Vec::<String>::new());

    event_listener!(event: DisplayError => {
        let message = match event {
            DisplayError::String(message)=> message,
            DisplayError::AuthenticatedUnexpectedHttp(error) => match error {
                UnexpectedHttpError::Request(request_error) => request_error.into_error_message(),
                UnexpectedHttpError::Api(api_error) => api_error.into_error_message(),
            },
            DisplayError::UnauthenticatedUnexpectedHttp(error) => match error {
                UnexpectedHttpError::Request(request_error) => request_error.into_error_message(),
                UnexpectedHttpError::Api(api_error) => api_error.into_error_message(),
            },
            DisplayError::Login(error) => error.into_error_message(),
        };

        errors.push(message);
    });

    rsx! {
        div {
            class: "error-list bg-red-100 text-red-800 p-4 rounded",
            ul {
                for error in errors() {
                    li { "{error}" }
                }
            }
        }
    }
}


