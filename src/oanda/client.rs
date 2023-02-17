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

    pub async fn get_pricing(&self, instrument: String) -> Option<PricingResponse> {
        let request_url = format!("{}/v3/accounts/{}/pricing?instruments={}",self.url.clone(), self.account, instrument);

        let response: Result<reqwest::Response, reqwest::Error> = self.client
            .get(request_url)
            .bearer_auth(self.token.clone())
            .send()
            .await;

        //let ret: Result<PricingResponse, _> = response.unwrap().json().await;
        //ret.ok()
        if let Some(res) = response.ok() {
            return res.json().await.ok();
        }

        None
    }

    pub async fn get_open_positions(&self,) -> Option<OpenPositionsResponse> {
        let request_url = format!("{}/v3/accounts/{}/openPositions",self.url.clone(), self.account);

        let response: Result<reqwest::Response, reqwest::Error> = self.client
            .get(request_url)
            .bearer_auth(self.token.clone())
            .send()
            .await;
//        if response.is_err() {
//            respon