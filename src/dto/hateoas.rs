use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct Link {
    pub href: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<&'static str>,
}

impl Link {
    pub fn get(href: String) -> Self {
        Self {
            href,
            method: Some("GET"),
        }
    }
    pub fn post(href: String) -> Self {
        Self {
            href,
            method: Some("POST"),
        }
    }
    pub fn put(href: String) -> Self {
        Self {
            href,
            method: Some("PUT"),
        }
    }
    pub fn patch(href: String) -> Self {
        Self {
            href,
            method: Some("PATCH"),
        }
    }
    pub fn delete(href: String) -> Self {
        Self {
            href,
            method: Some("DELETE"),
        }
    }
}

pub type Links = HashMap<String, Link>;
