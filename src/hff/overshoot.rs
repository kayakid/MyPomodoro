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
            self.start = self.peak;
            self.peak = x;
            self.current = x;
            // setting the direction right?
            self.direction = self.maxOS().signum() as i64;
        } else if cos.abs() > maxOS.abs() {
            self.peak = x;
        }
        self.current = x;
        //self.liquidity.update(self.maxOS());
    }
}

// Would be nice to have a maxiaml segment analysis version, memoizing sequnces of unexpected scores
#[derive(Debug, Clone, Copy)]
pub struct LocalLiquidity {
    pub alpha: f64, // exponential moving average rate
    pub liq: f64,
    pub surprise: f64,
    state: i64, // state keeps the status over previous event (-2, -1, 1, 2)
}

impl LocalLiquidity {
    pub fn new(alpha: f64) -> Self {
        Self {
            alpha: alpha,
            liq: 0.5,
            surprise: 0.0,
            state: 0,
        }
    }

    pub fn update(&mut self, maxOS: f64) -> f64 {
        let os = maxOS.signum() * (maxOS.abs() - 1.0);
        let H1: f64 = 0.28;
        let H2sr: f64 = 0.66;
        //
        let mut surp = 0.0;
        if os > 2.52 && self.state != 2 {
            surp = 2.52;
            self.state = 2;
        } else if os < -2.52 && self.state != -2 {
            surp = 2.52;
            self.state = -2;
        } else if os > 0.0 && self.state <= 0 {
            surp = 0.083;
            self.state = 1;
        } else if os < 0.0 && self.state >= 0 {
            surp = 0.083;
            self.state = -1;
        } else {
            return self.liq;
        }

        self.surprise = self.alpha * self.surprise + (1.0 - self.alpha) * surp; // -log(exp(-os))
        self.liq = 1.0 - Liquidity::CumNorm((self.surprise - H1) / H2sr);
        self.liq
    }
}

pub struct Liquidity {
    pub alpha: f64, // exponential moving average rate
    pub liq: f64,
    pub surprise: f64,
}

impl Liquidity {
    pub fn new(alpha: f64) -> Self {
        Self {
            alpha: alpha,
            liq: 0.5,
            surprise: 0.0,
        }
    }

    pub fn update(&mut self, maxOS: f64) -> f64 {
        let os = maxOS.abs() - 1.0;
        let H1: f64 = 1.0;
        let H2: f64 = 1.0;
        self.surprise = self.alpha * self.surprise + (1.0 - self.alpha) * os; // -log(exp(-os))
        self.liq = 1.0 - Liquidity::CumNorm((self.surprise - H1) / H2.sqrt());
        self.liq
    }

    pub fn CumNorm(x: f64) -> f64 {
        if x > 6.0 {
            return 1.0;
        }
        if x < -6.0 {
            return 0.0;
        }

        let b1 = 0.31938153;
        let b2 = -0.356563782;
        let b3 = 1.781477937;
        let b4 = -1.821255978;
        let b5 = 1.330274429;
        let p = 0.2316419;
        let c2 = 0.3989423;

        let a = x.abs();
        let t = 1.0 / (1.0 + a * p);
        let b = c2 * ((-x) * (x / 2.0)).exp();
        let mut n = ((((b5 * t + b4) * t + b3) * t + b2) * t + b1) * t;
        n = 1.0 - b * n;

        if x < 0.0 {
            n = 1.0 - n;
        }
        n
    }
}
