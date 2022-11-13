use serde::{Serialize,Deserialize};

// classes from the spectrum service
#[derive(Deserialize, Debug)]
pub struct os {
    pub instrument: String,
    pub scale: f64,
    pub direction: i64,
    pub start: f64,
    pub peak: f64,
    pub current: f64,
}

#[derive(Deserialize, Debug)]
pub struct sp {
    pub scales: Vec<f64>,
    pub overshoots: Vec<os>,
}

impl sp {
    pub fn to_spectrum(&self) -> Spectrum {
        Spectrum {
            overshoots: self.overshoots.iter().map(|o| (o.scale, Overshoot{
                scale: o.scale,
                direction: o.direction,
                start: o.start,
                peak: o.peak,
                current: o.current,
            })).collect(),
        }
    }
}

pub struct SpectrumClient {
    url: String,
    client: reqwest::Client,
}

impl SpectrumClient {
    pub fn new(url: String) -> SpectrumClient {
        let ret = SpectrumClient {
            url: url,
            client: reqwest::Client::new(),
        };
        ret
    }


    pub async fn get(&self) -> Option<sp> {
        let request_url = format!("{}",self.url.clone());

        let response: Result<reqwest::Response, reqwest::Error> = self.client
            .get(request_url)
            .send()
            .await;

        //let ret: Result<PricingResponse, _> = response.unwrap().json().await;
        //ret.ok()
        if let Some(res) = response.ok() {
            return res.json().await.ok();
        }

        None
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Overshoot {
    pub scale: f64,
    pub direction: i64,
    pub start: f64,
    pub peak: f64,
    pub current: f64,
    //pub liquidity:     LocalLiquidity,
}

#[derive(Debug, Clone)]
pub struct Spectrum {
    pub overshoots: Vec<(f64, Overshoot)>,
}

impl Spectrum {
    pub fn new(scales: Vec<f64>) -> Self {
        Self {
            overshoots: scales
                .iter()
           