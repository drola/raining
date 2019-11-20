extern crate reqwest;
use regex::Regex;
use serde::{Deserialize, Serialize};
#[macro_use]
extern crate log;

//use chrono::prelude::*;

#[derive(Serialize, Deserialize, std::fmt::Debug)]
struct RadarPercipitationResponseItem {
    mode: String,
    path: String,
    date: String,
    hhmm: String,
    bbox: String,
    width: String,
    height: String,
    valid: String,
}

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

fn pxToPercipitation(px: &image::Rgba<u8>) -> f32 {
    /**
     * RGBA:
     * (0, 0, 0)        :  0
     * (8, 70, 254, 255): 15
     * (0, 120, 254)    : 18
     * (0, 174, 253)    : 21
     * (0, 220, 254)    : 24
     * (4, 216, 131)    : 27
     * (66, 235, 66)    : 30
     * (108, 249, 0)    : 33
     * (184, 250, 0)    : 36
     * (249, 250, 1)    : 39
     * (254, 198, 0)    : 42
     * (254, 132, 0)    : 45
     * (255, 62, 1)     : 48
     * (211, 0, 0)      : 51
     * (181, 3, 3)      : 54
     * (203, 0, 204)    : 57
     */

    // https://docs.rs/color_processing/0.4.1/color_processing/struct.Color.html
    /**
     * HSV
     * (255, 97, 99)  15
     * (212, 100, 99) 18
     * (199, 100, 99) 21
     * (188, 100, 99) 24
     * (156, 98, 85)  27
     * (120, 72, 92)  30
     * (94, 100, 98)  33
     * (76, 100, 98)  36
     * (60, 99, 98)   39
     * (47, 100, 99)  42
     * (31, 100, 99)  45
     * (14, 99, 100)  48
     * (0, 100, 83)   51
     * (0, 98, 71)    54
     * (300, 100, 80) 57
     */

    return 0.0;
}


impl WeatherMap {
    fn new(lat: f32, lon: f32) -> GeoPoint {
        return GeoPoint { lat: lat, lon: lon };
    }
    
    fn value(&self, point: GeoPoint) -> f32 {
        let x = ((point.lon - self.bbox.lon1) / (self.bbox.lon2 - self.bbox.lon1)
            * self.image.width() as f32)
            .round() as u32;
        let y = ((point.lat - self.bbox.lat1) / (self.bbox.lat2 - self.bbox.lat1)
            * self.image.height() as f32)
            .round() as u32;
        let px = *self.image.get_pixel(x, y);
        return px[3] as f32 / 255.0;
    }
}

/*fn to_radar_percipitation_map(ri: &RadarPercipitationResponseItem) -> WeatherMap {
    return WeatherMap {};
}*/

fn to_image_url(rel_path: &str) -> String {
    return format!("{}{}", "http://www.meteo.si", rel_path);
}

fn download_image(url: &str) -> std::result::Result<image::RgbaImage, Box<dyn std::error::Error>> {
    let mut resp = reqwest::get(url)?;
    let mut buf: Vec<u8> = vec![];
    resp.copy_to(&mut buf)?;

    return Ok(image::io::Reader::new(std::io::Cursor::new(buf))
        .with_guessed_format()?
        .decode()?
        .to_rgba());
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Hello");
    warn!("Warn example");
    error!("Err example");

    let _ = std::thread::Builder::new()
        .name("data_fetch".to_string())
        .spawn(move || {
            info!("Hello from data_fetch thread");
        })?
        .join();
    info!("Back in main thread");

    //TODO:
    // - data fetch thread should check for new data every 10min and download all new images
    // - this data should be stored in the shared storage between the threads
    // - purge old data

    /*let resp: Vec<RadarPercipitationResponseItem> = reqwest::get(
        "http://www.meteo.siuploads/probase/www/nowcast/inca/inca_si0zm_data.json?prod=si0zm",
    )?
    .json()?;
    let maps: Vec<RadarPercipitationMap> = resp.iter().map(&to_radar_percipitation_map).collect();
    println!("{:#?}", resp); */
    Ok(())
}

fn parse_datetime(
    dt: &str,
) -> std::result::Result<chrono::DateTime<chrono::FixedOffset>, chrono::ParseError> {
    return chrono::DateTime::parse_from_rfc3339(dt);
}

fn parse_bbox(s: &str) -> std::result::Result<Bbox, std::num::ParseFloatError> {
    let re = Regex::new(
        r"^(?P<lat1>\d+\.\d+),(?P<lon1>\d+\.\d+),(?P<lat2>\d+\.\d+),(?P<lon2>\d+\.\d+)$",
    )
    .unwrap();
    let caps = re.captures(s).unwrap();
    return Ok(Bbox {
        lat1: caps.name("lat1").unwrap().as_str().parse()?,
        lon1: caps.name("lon1").unwrap().as_str().parse()?,
        lat2: caps.name("lat2").unwrap().as_str().parse()?,
        lon2: caps.name("lon2").unwrap().as_str().parse()?,
    });
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
