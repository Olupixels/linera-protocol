// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Types used when performing HTTP requests.

use custom_debug_derive::Debug;
use linera_witty::{WitLoad, WitStore, WitType};
use serde::{Deserialize, Serialize};

use crate::hex_debug;

/// An HTTP request.
#[derive(Clone, Debug, Eq, PartialEq, WitLoad, WitStore, WitType)]
#[witty(name = "http-request")]
pub struct Request {
    /// The [`Method`] used for the HTTP request.
    pub method: Method,

    /// The URL this request is intended to.
    pub url: String,

    /// The headers that should be included in the request.
    pub headers: Vec<Header>,

    /// The body of the request.
    #[debug(with = "hex_debug")]
    pub body: Vec<u8>,
}

impl Request {
    /// Creates an HTTP GET [`Request`] for a `url`.
    pub fn get(url: impl Into<String>) -> Self {
        Request {
            method: Method::Get,
            url: url.into(),
            headers: vec![],
            body: vec![],
        }
    }

    /// Creates an HTTP POST [`Request`] for a `url` with a `payload` that's arbitrary bytes.
    pub fn post(url: impl Into<String>, payload: impl Into<Vec<u8>>) -> Self {
        Request {
            method: Method::Post,
            url: url.into(),
            headers: vec![],
            body: payload.into(),
        }
    }

    /// Creates an HTTP POST [`Request`] for a `url` with a body that's the `payload` serialized to
    /// JSON.
    pub fn post_json(
        url: impl Into<String>,
        payload: &impl Serialize,
    ) -> Result<Self, serde_json::Error> {
        Ok(Request {
            method: Method::Post,
            url: url.into(),
            headers: vec![Header::new("Content-Type", b"application/json")],
            body: serde_json::to_vec(payload)?,
        })
    }

    /// Adds a header to this [`Request`].
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<Vec<u8>>) -> Self {
        self.headers.push(Header::new(name, value));
        self
    }
}

/// The method used in an HTTP request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, WitLoad, WitStore, WitType)]
#[witty(name = "http-method")]
pub enum Method {
    /// A GET request.
    Get,

    /// A POST request.
    Post,

    /// A PUT request.
    Put,

    /// A DELETE request.
    Delete,

    /// A HEAD request.
    Head,

    /// A OPTIONS request.
    Options,

    /// A CONNECT request.
    Connect,

    /// A PATCH request.
    Patch,

    /// A TRACE request.
    Trace,
}

#[cfg(with_reqwest)]
impl From<Method> for reqwest::Method {
    fn from(method: Method) -> Self {
        match method {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
            Method::Head => reqwest::Method::HEAD,
            Method::Options => reqwest::Method::OPTIONS,
            Method::Connect => reqwest::Method::CONNECT,
            Method::Patch => reqwest::Method::PATCH,
            Method::Trace => reqwest::Method::TRACE,
        }
    }
}

/// A response for an HTTP request.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, WitLoad, WitStore, WitType)]
#[witty(name = "http-response")]
pub struct Response {
    /// The status code of the HTTP response.
    pub status: u16,

    /// The headers included in the response.
    pub headers: Vec<Header>,

    /// The body of the response.
    #[debug(with = "hex_debug")]
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

impl Response {
    /// Creates an HTTP [`Response`] with a user defined `status_code`.
    pub fn new(status_code: u16) -> Self {
        Response {
            status: status_code,
            headers: vec![],
            body: vec![],
        }
    }

    /// Creates an HTTP [`Response`] with an OK status code and the provided `body`.
    pub fn ok(body: impl Into<Vec<u8>>) -> Self {
        Response {
            status: 200,
            headers: vec![],
            body: body.into(),
        }
    }

    /// Creates an HTTP [`Response`] with an Unauthorized status code.
    pub fn unauthorized() -> Self {
        Response {
            status: 401,
            headers: vec![],
            body: vec![],
        }
    }

    /// Adds a header to this [`Response`].
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<Vec<u8>>) -> Self {
        self.headers.push(Header::new(name, value));
        self
    }
}

/// A header for an HTTP request or response.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, WitLoad, WitStore, WitType)]
#[witty(name = "http-header")]
pub struct Header {
    /// The header name.
    pub name: String,

    /// The value of the header.
    #[debug(with = "hex_debug")]
    #[serde(with = "serde_bytes")]
    pub value: Vec<u8>,
}

impl Header {
    /// Creates a new [`Header`] with the provided `name` and `value`.
    pub fn new(name: impl Into<String>, value: impl Into<Vec<u8>>) -> Self {
        Header {
            name: name.into(),
            value: value.into(),
        }
    }
}
