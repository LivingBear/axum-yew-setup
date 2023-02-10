
use yew::prelude::*;

use wasm_bindgen_futures::spawn_local;

use crate::{models::components::server_request::ServerRequest, components::generic::error::no_response::no_server_response};
use crate::components::generic::success::some_response::some_server_response;
use crate::components::generic::error::some_error_response::some_error_reponse;

#[function_component(EmailData)]
pub fn email_data() -> Html {
    let data = use_state(|| None);

    // Request `/api/hi` once
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let mut request = ServerRequest { data: None, url: "/api/get_email_list".to_string() };
                    request.fetch_data().await;
                    data.set(request.data);
                });
            }

            || {}
        });
    }

    match data.as_ref() {
        None => { no_server_response() }
        Some(Ok(data)) => { some_server_response(&data.to_string()) }
        Some(Err(err)) => {some_error_reponse(&err.to_string())}
    }

}

