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
                .map(|scale| (*scale, Overshoot::new(*scale)))
                .collect(),
        }
    }

    pub fn update(&mut self, x: f64) {
        for os in &mut self.overshoots {
            os.1.update(x);
        }
    }
    pub fn maxOS(&self) -> Vec<f64> {
        self.overshoots.iter().map(|os| os.1.maxOS()).collect()
    }
}

impl Overshoot {
    pub fn new(scale: f64) -> Self {
        Self {
            scale: scale,
            direction: 1,
            start: 1.3745,
            peak: 1.3745,
            current: 1.3745,
            //liquidity: LocalLiquidity::new(0.95),
        }
    }

    pub fn maxOS(&self) -> f64 {
        100.0 * (self.peak - self.start) / self.start / self.scale
    }
    pub fn reversal(&self) -> f64 {
        100.0 * (self.current - self.peak) / self.peak / self.scale
    }

    // this is like the update but we return a state corresponding to reversal (±1, or preset threshold crossing)
    pub fn updateWithState(&mut self, x: f64, omega: f64) -> i64 {
        let dir = self.direction;
        let maxOS = self.maxOS();
        self.update(x);
        if dir != self.direction {
            return self.direction;
        }
        if maxOS > 0.0 && maxOS < omega && self.maxOS() >= omega {
            return 2;
        }
        if maxOS < 0.0 && maxOS > -omega && self.maxOS() <= -omega {
            return -2;
        }
        0
    }

    pub fn update(&mut self, x: f64) {
        //new := *ovs
        let cos = 100.0 * (x - self.start) / self.start / self.scale;
        let eDist = 100.0 * (x - self.peak) / self.peak / self.scale;
        let maxOS = self.maxOS();
        // if reversal...
        if cos * eDist < 0.0 && eDist.abs() > 1.0 {
            self.direction = -self.direction;
       