mod api;

pub struct AerisWeatherProvider {
    base_url: String,
    client_id: String,
    client_secret: String,
}

impl Default for AerisWeatherProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AerisWeatherProvider {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.aerisapi.com/".to_owned(),
            client_id: "".to_owned(),
            client_secret: "".to_owned(),
        }
    }

    pub fn with_base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn with_credentials<S: Into<String>>(mut self, client_id: S, client_secret: S) -> Self {
        self.client_id = client_id.into();
        self.client_secret = client_secret.into();
        self
    }
}

#[cfg(test)]
mod tests;
