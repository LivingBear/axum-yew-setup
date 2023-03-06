use gloo_net::http::{Request, Method};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Clone)]
pub struct RegistrationRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}
#[derive(Clone)]

pub struct ServerRequest {
    pub data: Option<Result<String, String>>,
    pub url: String,
    pub method: Method,
    pub auth_token: Option<String>,
    pub params: Option<Vec<(String, String)>>,
    pub data_body: Option<RegistrationRequest>,
}

impl ServerRequest {
    pub async fn fetch_data(&mut self) {
        let request = self.build_request();
        let resp = request.send().await.unwrap();
        self.data = {
            if !resp.ok() {
                Some(Err(format!(
                    "Error fetching data {} ({})",
                    resp.status(),
                    resp.status_text()
                )))
            } else {
                Some(resp.text().await.map_err(|err| err.to_string()))
            }
        };
    }

    fn build_request(&self) -> Request {
        let mut url = self.url.clone();
        if let Some(params) = &self.params {
            let query = params
                .iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect::<Vec<String>>()
                .join("&");
            if !query.is_empty() {
                url.push_str(&format!("?{}", query));
            }
        }

        let mut request_builder = match &self.method {
            Method::GET => Request::get(&url),
            Method::POST => { 
                let mut req = Request::post(&url);
                if let Some(body) = &self.data_body {
                    req = req.header("Content-Type", "application/json")
                             .body(json!(body).to_string());
                }
                req
            },
            Method::PUT => Request::put(&url),
            Method::DELETE => Request::delete(&url),
            Method::PATCH => Request::patch(&url),
            _ => unreachable!(),
        };


        request_builder
    }
}