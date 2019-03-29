use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FlightRecord {
    carrier: String,
    origin: String,
    destination: String,
    departure_delay: Option<i32>,
    arrival_delay: Option<i32>,
    cancelled: bool,
    distance: i32,
}

impl FlightRecord {
    pub fn carrier(&self) -> &str {
        &self.carrier
    }

    pub fn origin(&self) -> &str {
        &self.origin
    }

    pub fn destination(&self) -> &str {
        &self.destination
    }

    pub fn departure_delay(&self) -> &Option<i32> {
        &self.departure_delay
    }

    pub fn arrival_delay(&self) -> &Option<i32> {
        &self.arrival_delay
    }
}
