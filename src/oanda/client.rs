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
    pub fn new(url: String, account: String, token: String) -> Client {
        let ret = Client {
            token: token,
            account: account,
            url: url,
            client: reqwest::Client::new(),
        };
        ret
    }

    pub async fn get_pricing(&self, instrument: String) -