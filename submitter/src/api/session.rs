use reqwest::{header, RequestBuilder};

use super::API;

impl API {
    pub fn has_session(&self) -> bool {
        self.session.is_some()
    }

    pub fn use_session(&self, req: RequestBuilder) -> RequestBuilder {
        if let Some(sess) = &self.session {
            req.header(header::COOKIE, format!("PHPSESSID={}", sess))
        } else {
            req
        }
    }
}
