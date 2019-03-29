mod command;
mod data;

use crate::{command::Command, data::FlightRecord};
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    execute(&Command::from_args());
}

fn execute(command: &Command) {
    if let Ok(mut reader) = csv::Reader::from_path(command.path()) {
        let flight_records: Vec<_> = reader
            .deserialize()
            .flat_map(|record| record.ok())
            .collect();

        print_delays_by_airline(&flight_records, &command.origin(), &command.destination());
        print_worst_airports(&flight_records);
    }
}

fn print_delays_by_airline(records: &[FlightRecord], origin: &str, destination: &str) {
    let origin = origin.to_uppercase();
    let destination = destination.to_uppercase();
    let records = records
        .iter()
        .filter(|record| is_valid_flight(record, &origin, &destination))
        .fold(HashMap::new(), |mut map, record| {
            map.entry(record.carrier())
                .or_insert(Vec::new())
                .push(record);
            map
        });

    for (key, group) in records {
        print_delays(key, &group)
    }
}

fn print_delays(airline: &str, records: &[&FlightRecord]) {
    let (count, total, max) = records
        .iter()
        .fold((0, 0, 0), |(count, total, max), record| {
            (
                count + 1,
                total + record.arrival_delay().unwrap_or(0),
                std::cmp::max(max, record.arrival_delay().unwrap_or(0)),
            )
        });

    println!(
        "{} Avg/Max delay: {:.2}/{}",
        airline,
        total as f32 / count as f32,
        max
    );
}

fn print_worst_airports(records: &[FlightRecord]) {
    let origins = records.iter().fold(HashMap::new(), |mut map, record| {
        map.entry(record.origin())
            .or_insert(Vec::new())
            .push(record);
        map
    });

    let mut origin_delays: Vec<_> = origins
        .iter()
        .map(|(key, group)| {
            let (count, total) = group.iter().fold((0, 0), |(count, total), record| {
                (count + 1, total + record.departure_delay().unwrap_or(0))
            });
            (key, total as f32 / count as f32)
        })
        .collect();

    origin_delays.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
    if let Some(worst_delay) = origin_delays.last() {
        println!(
            "Worst average departure delay: {} ({:.2})",
            worst_delay.0, worst_delay.1
        );
    }

    let destinations = records.iter().fold(HashMap::new(), |mut map, record| {
        map.entry(record.origin())
            .or_insert(Vec::new())
            .push(record);
        map
    });

    let mut destination_delays: Vec<_> = destinations
        .iter()
        .map(|(key, group)| {
            let (count, total) = group.iter().fold((0, 0), |(count, total), record| {
                (count + 1, total + record.arrival_delay().unwrap_or(0))
            });
            (key, total as f32 / count as f32)
        })
        .collect();

    destination_delays.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    if let Some(worst_delay) = destination_delays.last() {
        println!(
            "Worst average arrival delay: {} ({:.2})",
            worst_delay.0, worst_delay.1
        );
    }
}

fn is_valid_flight(record: &FlightRecord, origin: &str, destination: &str) -> bool {
    record.origin().to_uppercase() == origin && record.destination().to_uppercase() == destination
}
