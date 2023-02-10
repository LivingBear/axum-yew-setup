use gloo_net::http::Request;
pub struct ServerRequest {
    pub data: Option<Result<String, String>>,
    pub url: String
}

impl ServerRequest {
    pub async fn fetch_data(&mut self) {
        let resp = Request::get(&self.url).send().await.unwrap();
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
}
