use crate::constants::{
    IMG_HEIGHT_HALF, IMG_WIDTH_HALF, LAT_BOUNDS, LAT_CENTER, LON_BOUNDS, LON_CENTER, LON_WIDTH_HALF, LAT_HEIGHT_HALF,
};
use anyhow::{anyhow, Ok, Result};
use serde::Deserialize;
use turtle::Point;

#[derive(Debug, Deserialize)]
pub struct HurricanePoint {
    Date: String,
    Time: String,
    Lat: f64,
    Lon: f64,
    Wind: u64, // miles per hour
    Pressure: u64,
}

impl HurricanePoint {
    pub fn determine_category(&self) -> usize {
        if self.Wind < 74 {
            0
        } else if self.Wind < 96 {
            1
        } else if self.Wind < 111 {
            2
        } else if self.Wind < 130 {
            3
        } else if self.Wind < 157 {
            4
        } else {
            5
        }
    }

    pub fn translate_to_turtle_grid(&self) -> Result<Point> {
        if LAT_BOUNDS.0 > self.Lat
            || LAT_BOUNDS.1 < self.Lat
            || LON_BOUNDS.0 > self.Lon
            || LON_BOUNDS.1 < self.Lon
        {
            return Err(anyhow!("The Hurricane Point is out of bounds for output image latitude and longitude. Point: Lat {} Lon {}", self.Lat, self.Lon));
        }

        let shifted_lon = self.Lon - LON_CENTER;
        let shifted_lat = self.Lat - LAT_CENTER;

        let scaled_lon = shifted_lon * (IMG_WIDTH_HALF / LON_WIDTH_HALF).abs();
        let scaled_lat = shifted_lat * (IMG_HEIGHT_HALF / LAT_HEIGHT_HALF).abs();

        Ok(Point {
            x: scaled_lon,
            y: scaled_lat,
        })
    }
}

pub fn read_csv_data_from_file(path: &str) -> Result<Vec<HurricanePoint>> {
    let mut reader = csv::Reader::from_path(path)?;
    let deserialized_data = reader
        .deserialize()
        .map(|row| row.map_err(|e| anyhow!(e)))
        .collect();

    deserialized_data
}
