#[derive(Clone, Debug, Default)]
pub struct Config {
    host: Option<String>,
    api_key: Option<String>,
    secret_key: Option<String>,
}

impl Config {
    fn host(&mut self, host: String) -> &mut Self {
        self.host = Some(host);
        self
    }

    fn api_key(&mut self, api_key: String) -> &mut Self {
        self.api_key = Some(api_key);
        self
    }

    fn secret_key(&mut self, secret_key: String) -> &mut Self {
        self.secret_key = Some(secret_key);
        self
    }
}

#[cfg(test)]
mod tests {}
