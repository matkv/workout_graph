use std::{error::Error, io, process}
//use serde::{Deserialize}

#[derive(Debug, serde::Deserialize)]
struct Activity {
    title: String
}

fn main() {
    read_csv();
}

fn read_csv() -> Result<(), Box<dyn Error>> {
    let mut activities = csv::Reader::from_path("activities.csv");

    for activity in activities.unwrap().deserialize() {
        let workout: Activity = activity?;
        println!("{:?}", workout);
    }

    Ok(())
}
