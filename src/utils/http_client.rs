use reqwest::Client;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn get(&self, url: &str) -> reqwest::RequestBuilder {
        self.client.get(url)
    }
}
