use yew::prelude::*;

use wasm_bindgen_futures::spawn_local;

use crate::models::components::server_request::ServerRequest;

#[function_component(HiServer)]
pub fn hi_server() -> Html {
    let data = use_state(|| None);

    // Request `/api/hi` once
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let mut request = ServerRequest { data: None, url: "/api/hi".to_string() };
                    request.fetch_data().await;
                    data.set(request.data);
                });
            }

            || {}
        });
    }

    match data.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <div>{"Got server response: :) "}{data}</div>
            }
        }
        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from server: "}{err}</div>
            }
        }
    }

}
