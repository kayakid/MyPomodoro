use super::*;
use std::error::Error;
use serde_json::json;

pub struct Client {
    token: String,
    url: String,
    account: String,
    client: reqwest::Client,
}

impl Client {
    p