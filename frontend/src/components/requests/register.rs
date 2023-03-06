use crate::components::generic::success::some_response::some_server_response;
use crate::models::components::server_request::RegistrationRequest;
use gloo_net::http::Method::POST;

use yew::prelude::*;

use wasm_bindgen_futures::spawn_local;

use crate::components::generic::error::some_error_response::some_error_reponse;
use crate::{
    components::generic::error::no_response::no_server_response,
    models::components::server_request::ServerRequest,
};

#[function_component(RegisterRequest)]
pub fn register_request() -> Html {
    let data = use_state(|| None);
    // Request `/api/register` once
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let mut request = ServerRequest {
                        data: None,
                        url: "/api/register".to_string(),
                        method: POST,
                        // auth_token: Some("my-token".to_string()),
                        auth_token: None,
                        // params: Some(vec![("foo".to_string(), "bar".to_string())]),
                        params: None,
                        // Take in specifically from the component that has called this function with the data inside the fields.
                        data_body: Some(RegistrationRequest { username: "danraine".to_owned(), email: "dan@gcpp.gold".to_owned(), password: "password".to_owned()}),
                    };
                    request.fetch_data().await;
                    data.set(request.data);
                });
            }
            || {}
        });
    }

    match data.as_ref() {
        None => no_server_response(),
        Some(Ok(data)) => some_server_response(&data.to_string()),
        Some(Err(err)) => some_error_reponse(&err.to_string()),
    }
}
