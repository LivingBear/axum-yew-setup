use yew::prelude::*;

pub fn some_server_response(data: &String) -> Html {
    html! {
        <div>{"Received a response from the server: "}{data}</div>
    }

}