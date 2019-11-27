extern crate reqwest;
use regex::Regex;
use serde::{Deserialize, Serialize};
#[macro_use]
extern crate log;
mod px_to_precipitation;
mod meteo_data;
mod web_server;

//use chrono::prelude::*;


#[derive(PartialEq, std::fmt::Debug)]
struct Bbox {
    lat1: f32,
    lat2: f32,
    lon1: f32,
    lon2: f32,
}

struct WeatherMap {
    //  datetime: chrono::DateTime<chrono::FixedOffset>,
    //  image_url: String,
    bbox: Bbox,
    image: image::RgbaImage,
}

struct GeoPoint {
    lat: f32,
    lon: f32,
}


impl GeoPoint {
    fn new(lat: f32, lon: f32) -> GeoPoint {
        GeoPoint { lat, lon }
    }
}

impl WeatherMap {
    fn value(&self, point: GeoPoint) -> f32 {
        let x = ((point.lon - self.bbox.lon1) / (self.bbox.lon2 - self.bbox.lon1)
            * self.image.width() as f32)
            .round() as u32;
        let y = ((point.lat - self.bbox.lat1) / (self.bbox.lat2 - self.bbox.lat1)
            * self.image.height() as f32)
            .round() as u32;
        let px = *self.image.get_pixel(x, y);
        px[3] as f32 / 255.0
    }
}

/*fn to_radar_percipitation_map(ri: &RadarPercipitationResponseItem) -> WeatherMap {
    return WeatherMap {};
}*/

fn to_image_url(rel_path: &str) -> String {
    format!("{}{}", "http://www.meteo.si", rel_path)
}

fn download_image(url: &str) -> std::result::Result<image::RgbaImage, Box<dyn std::error::Error>> {
    let mut resp = reqwest::get(url)?;
    let mut buf: Vec<u8> = vec![];
    resp.copy_to(&mut buf)?;

    Ok(image::io::Reader::new(std::io::Cursor::new(buf))
        .with_guessed_format()?
        .decode()?
        .to_rgba())
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Hello");
    warn!("Warn example");
    error!("Err example");

    let _ = std::thread::Builder::new()
        .name("web_server".to_string())
        .spawn(web_server::main)?
        .join();
    info!("Back in main thread");

    //TODO:
    // - data fetch thread should check for new data every 10min and download all new images
    // - this data should be stored in the shared storage between the threads
    // - purge old data

    /*let resp: Vec<RadarPercipitationResponseItem> = reqwest::get(
        "http://www.meteo.si/uploads/probase/www/nowcast/inca/inca_si0zm_data.json?prod=si0zm",
    )?
    .json()?;
    let maps: Vec<RadarPercipitationMap> = resp.iter().map(&to_radar_percipitation_map).collect();
    println!("{:#?}", resp); */
    Ok(())
}

fn parse_datetime(
    dt: &str,
) -> std::result::Result<chrono::DateTime<chrono::FixedOffset>, chrono::ParseError> {
    chrono::DateTime::parse_from_rfc3339(dt)
}

fn parse_bbox(s: &str) -> std::result::Result<Bbox, std::num::ParseFloatError> {
    let re = Regex::new(
        r"^(?P<lat1>\d+\.\d+),(?P<lon1>\d+\.\d+),(?P<lat2>\d+\.\d+),(?P<lon2>\d+\.\d+)$",
    )
    .unwrap();
    let caps = re.captures(s).unwrap();
    Ok(Bbox {
        lat1: caps.name("lat1").unwrap().as_str().parse()?,
        lon1: caps.name("lon1").unwrap().as_str().parse()?,
        lat2: caps.name("lat2").unwrap().as_str().parse()?,
        lon2: caps.name("lon2").unwrap().as_str().parse()?,
    })
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use chrono::offset::TimeZone;

    #[test]
    fn test_parse_datetime() {
        assert_eq!(
            parse_datetime("2019-11-03T12:10:00Z"),
            Ok(chrono::FixedOffset::east(0)
                .ymd(2019, 11, 3)
                .and_hms(12, 10, 00))
        );
    }

    #[test]
    fn test_parse_bbox() {
        assert_eq!(
            parse_bbox("44.67,12.1,47.42,17.44"),
            Ok(Bbox {
                lat1: 44.67,
                lon1: 12.1,
                lat2: 47.42,
                lon2: 17.44
            })
        );
    }

    #[test]
    fn test_to_image_url() {
        assert_eq!(
            to_image_url("/rel_path.png"),
            "http://www.meteo.si/rel_path.png"
        )
    }

    #[test]
    fn test_download_image() {
        let img = download_image("http://www.singlecolorimage.com/get/ea5130/100x100")
            .expect("cannot download img");
        let px = img.get_pixel(50, 50);
        assert_eq!(*px, image::Rgba([234u8, 81u8, 48u8, 255u8]));
    }
}
