#![feature(slice_patterns)]

extern crate csv;
extern crate rustc_serialize;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;

use rustc_serialize::{Decoder, Decodable};

#[derive(Debug)]
struct FlightRecord {
    carrier: String,
    origin: String,
    destination: String,
    departure_delay: Option<i32>,
    arrival_delay: Option<i32>,
    cancelled: bool,
    distance: i32,
}

impl Decodable for FlightRecord {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        let core = try!(FlightRecordCore::decode(d));
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

#[derive(Debug, RustcDecodable)]
struct FlightRecordCore {
    carrier: String,
    origin: String,
    destination: String,
    departure_delay: Option<i32>,
    arrival_delay: Option<i32>,
    cancelled: i32,
    distance: i32,
}

struct Command {
    path: String,
    origin: String,
    destination: String,
}

impl Command {
    fn new<T: Into<String>>(path: T, origin: T, destination: T) -> Command {
        Command {
            path: path.into(),
            origin: origin.into(),
            destination: destination.into(),
        }
    }

    fn path<'a>(&'a self) -> &'a Path {
        Path::new(&self.path)
    }
}

#[derive(Debug)]
enum CommandParseError {
    InvalidArgCount,
    InvalidPath,
}

fn main() {
    match read_command() {
        Err(e) => {
            println!("bad command: {:?}", e);
            std::process::exit(1);
        },
        Ok(command) => execute(&command)
    }
}

fn execute(command: &Command) {
    if let Ok(mut reader) = csv::Reader::from_file(command.path()) {
        let flight_records: Vec<_> = reader.decode()
            .flat_map(|record| record.ok())
            .collect();

        print_delays_by_airline(&flight_records, &command.origin, &command.destination);
        print_worst_airports(&flight_records);
    }
}

fn print_delays_by_airline(records: &[FlightRecord], origin: &str, destination: &str) {
    let origin = origin.to_uppercase();
    let destination = destination.to_uppercase();
    let records = records.iter()
        .filter(|record| is_valid_flight(record, &origin, &destination))
        .fold(HashMap::new(), |mut map, record| {
            map.entry(&record.carrier).or_insert(Vec::new()).push(record);
            map
        });

    for (key, group) in records {
        print_delays(key, &group)
    }
}

fn print_delays(airline: &str, records: &[&FlightRecord]) {
    let (count, total, max) = records.iter()
        .fold((0, 0, 0), |(count, total, max), record| (
            count + 1,
            total + record.arrival_delay.unwrap_or(0),
            std::cmp::max(max, record.arrival_delay.unwrap_or(0))
        ));

    println!("{} Avg/Max delay: {:.2}/{}", airline, total as f32 / count as f32, max);
}

fn print_worst_airports(records: &[FlightRecord]) {
    let origins = records.iter().fold(HashMap::new(), |mut map, record| {
        map.entry(&record.origin).or_insert(Vec::new()).push(record);
        map
    });

    let mut origin_delays: Vec<_> = origins.iter().map(|(key, group)| {
        let (count, total) = group.iter().fold((0, 0), |(count, total), record| (
            count + 1,
            total + record.departure_delay.unwrap_or(0),
        ));
        (key, total as f32 / count as f32)
    }).collect();

    origin_delays.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
    if let Some(worst_delay) = origin_delays.last() {
        println!("Worst average departure delay: {} ({:.2})", worst_delay.0, worst_delay.1);
    }

    let destinations = records.iter().fold(HashMap::new(), |mut map, record| {
        map.entry(&record.origin).or_insert(Vec::new()).push(record);
        map
    });

    let mut destination_delays: Vec<_> = destinations.iter().map(|(key, group)| {
        let (count, total) = group.iter().fold((0, 0), |(count, total), record| (
            count + 1,
            total + record.arrival_delay.unwrap_or(0),
        ));
        (key, total as f32 / count as f32)
    }).collect();

    destination_delays.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    if let Some(worst_delay) = destination_delays.last() {
        println!("Worst average arrival delay: {} ({:.2})", worst_delay.0, worst_delay.1);
    }
}

fn is_valid_flight(record: &FlightRecord, origin: &str, destination: &str) -> bool {
    record.origin.to_uppercase() == origin && record.destination.to_uppercase() == destination
}

fn read_command() -> Result<Command, CommandParseError> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match &args[..] {
        [ref path, ref origin, ref destination] => if Path::new(&path).exists() {
            Ok(Command::new(path.as_ref(), origin.as_ref(), destination.as_ref()))
        } else {
            Err(CommandParseError::InvalidPath)
        },
        _ => Err(CommandParseError::InvalidArgCount)
    }
}
