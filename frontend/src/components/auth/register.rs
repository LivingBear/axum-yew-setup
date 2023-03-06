use crate::{
    elements::text_input::{InputType, TextInput},
    models::components::server_request::{RegistrationRequest, ServerRequest},
};
use gloo_net::http::Method;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(RegisterComponent)]
pub fn register_form() -> Html {
    html! {
        <>
            <Register/>
        </>
    }
}

pub enum Msg {
    SetPassword(String),
    SetUsername(String),
    SetEmail(String),
    HandleSubmit,
}

#[derive(Debug, Default)]
pub struct Register {
    username: String,
    password: String,
    email: String,
    registration_result: Option<String>,
}

impl Register {
    fn verify_form(&self) -> Option<String> {
        // Verify that the username is unique and between 3-16 characters
        let is_username_valid = self.username.len() >= 3 && self.username.len() <= 16;
        let is_username_unique = /* your code to check if the username is unique */ true;
        let is_username_valid_and_unique = is_username_valid && is_username_unique;

        // Verify that the password is unique and between 8-32 characters
        let is_password_valid = self.password.len() >= 8 && self.password.len() <= 32;
        let is_password_unique = /* your code to check if the password is unique */ true;
        let is_password_valid_and_unique = is_password_valid && is_password_unique;

        // Verify that the email is a specific email
        let is_email_valid = self.email == "example@example.com";

        if !is_username_valid_and_unique {
            Some("Username must be unique and between 3-16 characters".to_string())
        } else if !is_email_valid {
            Some("Email must be 'example@example.com'".to_string())
        } else if !is_password_valid_and_unique {
            Some("Password must be unique and between 8-32 characters".to_string())
        } else {
            None
        }

    }

    fn handle_submit(&self) -> Callback<MouseEvent> {
        let is_form_valid = self.verify_form();
        
        if is_form_valid.is_none() {
            // Form is invalid, handle the error
            return Callback::noop();
        }

        let request = ServerRequest {
            url: "/api/register".to_string(), // Replace with the actual URL for registration
            method: Method::POST,
            auth_token: None, // Add the authentication token if required
            params: None,
            data_body: Some(RegistrationRequest {
                email: self.email.clone(),
                username: self.username.clone(),
                password: self.password.clone(),
            }),
            data: None,
        };

        let callback = move |_event: MouseEvent| {
            let mut request = request.clone();
            spawn_local(async move {
                request.fetch_data().await;
                match request.data {
                    Some(Ok(response)) => {
                        dbg!(response);
                    }
                    Some(Err(error)) => {
                        dbg!(error);

                        // Registration failed, handle the error
                    }
                    None => {
                        // Request failed, handle the error
                    }
                }
            });
        };

        Callback::from(callback)
    }
}



impl Component for Register {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetPassword(next_password) => {
                self.password = next_password;
                true
            },
            Msg::SetUsername(next_username) => {
                self.username = next_username;
                true
            },
            Msg::SetEmail(next_email) => {
                self.email = next_email;
                true
            },
            Msg::HandleSubmit => {
                let hows_form_looking = self.verify_form();
                if hows_form_looking.is_some() {
                    self.registration_result = Some(self.verify_form().unwrap());
                    true
                } else {
                    self.registration_result = None;
                    let submit_registration = _ctx.link().callback(|_| Msg::HandleSubmit);
                    submit_registration.emit(());
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_username_change = ctx.link().callback(Msg::SetUsername);
        let on_email_change = ctx.link().callback(Msg::SetEmail);
        let on_password_change = ctx.link().callback(Msg::SetPassword);
        let submit_registration = ctx.link().callback(|_| Msg::HandleSubmit);
        html! {
            <>
                <div class="registration-form">
                    <div class="form-group">
                        <label for="username-input">{ "Username:" }</label>
                        <TextInput on_change={on_username_change} value={self.username.clone()} input_type={InputType::Username} />
                    </div>
                    <div class="form-group">
                        <label for="email-input">{ "Email:" }</label>

                        <TextInput on_change={on_email_change} value={self.email.clone()} input_type={InputType::Email}/>
                    </div>
                    <div class="form-group">
                        <label for="password-input">{ "Password:" }</label>
                        <TextInput on_change={on_password_change} value={self.password.clone()} input_type={InputType::Password}/>
                    </div>
                    <div>
                        <button onclick={submit_registration}>{"Submit your registration"}</button>
                    </div>
                    {self.registration_result.clone()}
                </div>
            
            </>
        }
    }
}
