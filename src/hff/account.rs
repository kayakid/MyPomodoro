#[derive(Debug)]
pub struct Position {
    pub instrument: String,
    pub units: i64,
    pub price: Option<f64>,
}

#[derive(Debug)]
pub struct OrderFill {
    pub price: f64,
    pub units: i64,
}