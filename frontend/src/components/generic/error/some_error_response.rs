use yew::prelude::*;

pub fn some_error_reponse(err: &String) -> Html {
    html! {
        <div>{"Error requesting data from server: "}{err}</div>
    }

}
