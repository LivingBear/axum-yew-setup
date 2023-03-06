use yew::prelude::*;

use gloo_net::http::Method::GET;

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
                    let mut request = ServerRequest {
                        data: None,
                        url: "/api/hi".to_string(),
                        method: GET,
                        // auth_token: Some("my-token".to_string()),
                        auth_token: None,
                        // params: Some(vec![("foo".to_string(), "bar".to_string())]),
                        params: None,
                        data_body: None,
                    };

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
