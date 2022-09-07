use std::collections::HashMap;

use reqwest::{RequestBuilder, Response, StatusCode};
use serde_json::Value;

use crate::{
    error::{Error, SmartcarError},
    response::meta::{self, Meta},
};

pub(crate) trait MultiQuery {
    /// Build a vector of multiple query/value tuples
    fn vectorize(&self) -> Vec<(String, String)>;

    /// Build a string with multiple query values, minus the
    ///
    /// Note, the beginning of this string will NOT include
    /// the "?" (if it's the first query in the URL)
    /// OR the "&" beginnging/ending (if it proceeds another query)
    fn multi_query(&self) -> String {
        let mut query_string = String::from("");
        let query_vec = self.vectorize();

        for i in 0..query_vec.len() {
            if i != 0 {
                query_string.push_str("&");
            }

            let (q, v) = query_vec[i].to_owned();
            query_string.push_str(q.as_str());
            query_string.push_str("=");
            query_string.push_str(v.as_str());
        }

        query_string
    }
}

/// -> `Bearer <access_token>`
pub(crate) fn get_bearer_token_header(access_token: &str) -> String {
    format!("Bearer {}", access_token)
}

/// -> `Basic <base64('client_id:client_secrret')>`
pub(crate) fn get_basic_b64_auth_header(client_id: &str, client_secret: &str) -> String {
    let credentials = format!("{}:{}", client_id, client_secret);
    let encoded = base64::encode(credentials.as_bytes());
    format!("Basic {}", encoded.as_str())
}

pub(crate) enum HttpVerb {
    GET,
    POST,
    DELETE,
}

#[derive(Debug)]
pub(crate) struct SmartcarRequestBuilder {
    request: RequestBuilder,
}

impl SmartcarRequestBuilder {
    pub(crate) fn new(url: String, verb: HttpVerb) -> SmartcarRequestBuilder {
        let client = reqwest::Client::new();

        SmartcarRequestBuilder {
            request: match verb {
                HttpVerb::GET => client.get(url),
                HttpVerb::POST => client.post(url),
                HttpVerb::DELETE => client.delete(url),
            },
        }
    }
    pub(crate) fn add_header(mut self, header: &str, value: &str) -> Self {
        self.request = self.request.header(header, value);
        self
    }

    pub(crate) fn add_queries(mut self, qs: Vec<(&str, &str)>) -> Self {
        for q in qs {
            self.request = self.request.query(&q);
        }
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
            return Err(Error::SmartcarError(sc_err));
        }

        let meta = meta::generate_meta_from_headers(res.headers());

        Ok((res, meta))
    }
}
