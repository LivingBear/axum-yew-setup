// use gloo_timers::future::TimeoutFuture;
// use regex::Regex;
// use std::time::Duration;
// use wasm_bindgen::prelude::*;
// use yew::{html, Callback, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};
// use yewtil::NeqAssign;

// #[derive(Debug, Clone, PartialEq, Properties)]
// pub struct Props {
//     pub on_submit: Callback<(String, String, String)>,
// }

// pub enum Msg {
//     UsernameChanged(String),
//     EmailChanged(String),
//     PasswordChanged(String),
//     SubmitClicked,
// }

// pub struct RegisterForm {
//     props: Props,
//     link: ComponentLink<Self>,
//     username: String,
//     email: String,
//     password: String,
// }

// impl Component for RegisterForm {
//     type Message = Msg;
//     type Properties = Props;

//     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
//         Self {
//             props,
//             link,
//             username: "".to_string(),
//             email: "".to_string(),
//             password: "".to_string(),
//         }
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         match msg {
//             Msg::UsernameChanged(username) => {
//                 self.username = username;
//                 true
//             }
//             Msg::EmailChanged(email) => {
//                 self.email = email;
//                 true
//             }
//             Msg::PasswordChanged(password) => {
//                 self.password = password;
//                 true
//             }
//             Msg::SubmitClicked => {
//                 if self.validate_form() {
//                     self.props
//                         .on_submit
//                         .emit((self.username.clone(), self.email.clone(), self.password.clone()));
//                 }
//                 false
//             }
//         }
//     }

//     fn view(&self) -> Html {
//         html! {
//             <div class="registration-form">
//                 <div class="form-group">
//                     <label for="username-input">{ "Username:" }</label>
//                     <input
//                         type="text"
//                         id="username-input"
//                         class="form-control"
//                         value={self.username.clone()}
//                         oninput=self.link.callback(|e: InputData| Msg::UsernameChanged(e.value))
//                     />
//                 </div>
//                 <div class="form-group">
//                     <label for="email-input">{ "Email:" }</label>
//                     <input
//                         type="email"
//                         id="email-input"
//                         class="form-control"
//                         value={self.email.clone()}
//                         oninput=self.link.callback(|e: InputData| Msg::EmailChanged(e.value))
//                     />
//                 </div>
//                 <div class="form-group">
//                     <label for="password-input">{ "Password:" }</label>
//                     <input
//                         type="password"
//                         id="password-input"
//                         class="form-control"
//                         value={self.password.clone()}
//                         oninput=self.link.callback(|e: InputData| Msg::PasswordChanged(e.value))
//                     />
//                 </div>
//                 <button class="btn btn-primary" onclick=self.link.callback(|_| Msg::SubmitClicked)>{ "Register" }</button>
//             </div>
//         }
//     }
// }

// impl RegisterForm {
//     fn validate_form(&self) -> bool {
//         let username_re = Regex::new(r"^\w{3,16}$").unwrap();
//         let email_re = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();

//         if !username_re.is_match(&self.username) {
//             self.display_error_message("Username must be between 3 and 16 alphanumeric characters.");
//             return false;
//         }

//         if !email_re.is_match(&self.email) {
//             self.display_error_message("Email address must be valid and in the format of 'user@domain.com'.");

//             if self.password != self.confirm_password {
//                 self.display_error_message("Passwords do not match.");
//                 return false;
//             }
        
//             true
//         }
//     }
// }