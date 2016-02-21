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
        Ok(FlightRecord {
            carrier: try!(d.read_str()),
            origin: try!(d.read_str()),
            destination: try!(d.read_str()),
            departure_delay: d.read_i32().ok().map(|value| Some(value)).unwrap_or(None),
            arrival_delay: d.read_i32().ok().map(|value| Some(value)).unwrap_or(None),
            cancelled: try!(d.read_i32().map(|value| value != 0)),
            distance: try!(d.read_i32()),
        })
    }
}
