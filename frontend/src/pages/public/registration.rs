use yew::prelude::*;

use crate::components::{ auth::{register::RegisterComponent}};


#[function_component(RegistrationPage)]
pub fn registration_page() -> Html {
    html! {
        <>
        {"Hi from registration"}
        <RegisterComponent/>
        </>
    }
}
