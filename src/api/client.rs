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

    pub async fn request(&self) {
        let client = Client::new();
        let mut headers = header::HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
        headers.insert(header::AUTHORIZATION, format!("Bearer {}", self.token).parse().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let client = APIClient::new("https://c.koliosky.com", "IIKANJI_TOKEN");
    }
}