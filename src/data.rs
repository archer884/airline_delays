use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FlightRecord {
    #[serde(rename = "OP_UNIQUE_CARRIER")]
    carrier: String,
    #[serde(rename = "ORIGIN_CITY_NAME")]
    origin: String,
    #[serde(rename = "DEST_CITY_NAME")]
    destination: String,
    #[serde(rename = "DEP_DELAY")]
    departure_delay: Option<i32>,
    #[serde(rename = "ARR_DELAY")]
    arrival_delay: Option<i32>,
    #[serde(rename = "CANCELLED")]
    cancelled: u8,
    #[serde(rename = "CANCELLATION_CODE")]
    cancellation_code: Option<String>,
    #[serde(rename = "DISTANCE")]
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

    pub fn departure_delay(&self) -> Option<i32> {
        self.departure_delay
    }

    pub fn arrival_delay(&self) -> Option<i32> {
        self.arrival_delay
    }
}
