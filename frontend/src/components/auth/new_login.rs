use yew::prelude::*;
use crate::models::components::server_request::ServerRequest;

// Define the data structure for the login form input
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginData {
    pub email: String,
    pub username: String,
    pub password: String,
}

// Define the login component
pub struct LoginComponent {
    link: ComponentLink<Self>,
    login_data: LoginData,
    server_response: Option<Result<String, String>>,
}
pub enum Msg {
    UsernameChanged(String),
    PasswordChanged(String),
    Submit,
    ServerResponse(Result<String, String>),
}

impl Component for LoginComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}
