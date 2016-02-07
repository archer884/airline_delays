use rustc_serialize::{Decoder, Decodable};

#[derive(Debug)]
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

impl Decodable for FlightRecord {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        #[derive(Debug, RustcDecodable)]
        struct Core {
            carrier: String,
            origin: String,
            destination: String,
            departure_delay: Option<i32>,
            arrival_delay: Option<i32>,
            cancelled: i32,
            distance: i32,
        }

        let core = try!(Core::decode(d));
        Ok(FlightRecord {
            carrier: core.carrier,
            origin: core.origin,
            destination: core.destination,
            departure_delay: core.departure_delay,
            arrival_delay: core.arrival_delay,
            cancelled: core.cancelled != 0,
            distance: core.distance,
        })
    }
}
