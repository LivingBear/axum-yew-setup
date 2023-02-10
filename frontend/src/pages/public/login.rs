use yew::prelude::*;
use crate::components::auth::login::LoginComponent;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    html! {
        <LoginComponent/>
    }
}
