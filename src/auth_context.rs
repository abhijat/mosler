#[derive(Debug)]
pub struct AuthContext {
    pub server_address: String,
    pub auth_token: String,
}

pub struct ContextBuilder {
    pub server_address: String,
    pub auth_token: String,
}

impl ContextBuilder {
    pub fn new() -> Self {
        ContextBuilder { server_address: String::new(), auth_token: String::new() }
    }

    pub fn server_address(&self, address: &str) -> ContextBuilder {
        ContextBuilder { server_address: address.to_owned(), auth_token: self.auth_token.clone() }
    }

    pub fn auth_token(&self, token: &str) -> ContextBuilder {
        ContextBuilder { server_address: self.server_address.clone(), auth_token: token.to_owned() }
    }

    pub fn build(&self) -> AuthContext {
        AuthContext { server_address: self.server_address.clone(), auth_token: self.auth_token.clone() }
    }
}