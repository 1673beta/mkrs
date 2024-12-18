use reqwest::{Client, header};

pub struct APIClient {
    origin: String,
    token: String,
}

impl APIClient {
    pub fn new(origin: &str, token: &str) -> Self {
        APIClient {
            origin: origin.to_string(),
            token: token.to_string(),
        }
    }

    // TODO: ボディとURLを引数からいい感じに受け取るようにする
    pub async fn request(&self) -> Result<serde_json::Value, reqwest::Error> {
        let client = Client::new();
        let mut headers = header::HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
        headers.insert(header::AUTHORIZATION, format!("Bearer {}", self.token).parse().unwrap());
        let res = client.post(&self.origin)
            .headers(headers)
            .body("body")
            .send()
            .await?;
        
        let json: serde_json::Value = res.json().await?;
        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        let client = APIClient::new("https://c.koliosky.com", "IIKANJI_TOKEN");
        let res = client.request().await.unwrap();
    }
}