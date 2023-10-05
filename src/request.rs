use reqwest::{RequestBuilder, Response, StatusCode};
use serde_json::Value;
use std::collections::HashMap;

use crate::{
    error::{Error, SmartcarError},
    response::{meta, Meta},
};

pub(crate) trait MultiQuery {
    /// Build a vector of multiple query/value tuples
    fn vectorize(&self) -> Vec<(String, String)>;

    /// Build a string with multiple query/value pairs
    ///
    /// Note, the beginning of this string will NOT include
    /// an "?" or "&" in the beginning or end.
    fn multi_query(&self) -> String {
        let mut query_string = String::from("");
        let query_vec = self.vectorize();

        for (i, _) in query_vec.iter().enumerate() {
            if i != 0 {
                query_string.push('&');
            }

            let (q, v) = query_vec[i].to_owned();
            query_string.push_str(&q);
            query_string.push('=');
            query_string.push_str(&v);
        }

        query_string
    }
}

/// -> `Bearer <access_token>`
pub(crate) fn get_bearer_token_header(access_token: &str) -> String {
    format!("Bearer {access_token}")
}

/// -> `Basic <base64('client_id:client_secrret')>`
pub(crate) fn get_basic_b64_auth_header(client_id: &str, client_secret: &str) -> String {
    let credentials = format!("{}:{}", client_id, client_secret);
    let encoded = base64::encode(credentials.as_bytes());
    format!("Basic {}", &encoded)
}

pub enum HttpVerb {
    Get,
    Post,
    Delete,
}

#[derive(Debug)]
pub(crate) struct SmartcarRequestBuilder {
    request: RequestBuilder,
}

impl SmartcarRequestBuilder {
    pub(crate) fn new(url: &str, verb: HttpVerb) -> SmartcarRequestBuilder {
        let client = reqwest::Client::new();

        SmartcarRequestBuilder {
            request: match verb {
                HttpVerb::Get => client.get(url),
                HttpVerb::Post => client.post(url),
                HttpVerb::Delete => client.delete(url),
            },
        }
    }
    pub(crate) fn add_header(mut self, header: &str, value: &str) -> Self {
        self.request = self.request.header(header, value);
        self
    }

    pub(crate) fn add_query(mut self, query: &str, value: &str) -> Self {
        self.request = self.request.query(&[(query, value)]);
        self
    }

    pub(crate) fn add_body(mut self, body: Value) -> Self {
        self.request = self.request.json::<Value>(&body);
        self
    }

    pub(crate) fn add_form(mut self, form: HashMap<&str, &str>) -> Self {
        self.request = self.request.form(&form);
        self
    }

    pub(crate) async fn send(self) -> Result<(Response, Meta), Error> {
        let res = self.request.send().await?;

        if res.status() != StatusCode::OK {
            let sc_err = res.json::<SmartcarError>().await?;
            return Err(Error::SmartcarError(Box::new(sc_err)));
        }

        let meta = meta::generate_meta_from_headers(res.headers());

        Ok((res, meta))
    }
}
