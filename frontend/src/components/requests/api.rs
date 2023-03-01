use std::collections::HashMap;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct ApiComponent {
    data: Option<Result<String, String>>,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
}

enum Msg {
    FetchData(bool),
    ReceiveResponse(Result<String, String>),
}

impl Component for ApiComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            data: None,
            link,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData(auth_required) => {
                let url = if auth_required {
                    "/api/private"
                } else {
                    "/api/public"
                };

                let request = Request::get(url)
                    .header("Content-Type", "application/json")
                    .header("Authorization", if auth_required { "Bearer <your_token>" } else { "" })
                    .body(Nothing)
                    .unwrap();

                let callback = self.link.callback(
                    |response: Response<Json<Result<String, String>>>| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveResponse(data)
                    },
                );

                let task = FetchService::fetch(request, callback).unwrap();
                self.fetch_task = Some(task);
            }
            Msg::ReceiveResponse(response) => {
                self.data = Some(response);
                self.fetch_task = None;
            }
        }

        true
    }

    fn view(&self) -> Html {
        match &self.data {
            None => html! {
                <div>{"No server response"}</div>
            },
            Some(Ok(data)) => html! {
                <div>{"Got server response: :) "}{data}</div>
            },
            Some(Err(err)) => html! {
                <div>{"Error requesting data from server: "}{err}</div>
            },
        }
    }
}