use serde::{Deserialize, Serialize};
extern crate reqwest;
use regex::Regex;

#[derive(Serialize, Deserialize, std::fmt::Debug, std::cmp::PartialEq)]
struct RadarPrecipitationResponseItem {
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

fn parse_json(
    r: String,
) -> std::result::Result<Vec<RadarPrecipitationResponseItem>, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(r.as_str())?)
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

fn load_radar(
    site: Box<dyn super::site::SiteDownloader>,
) -> std::result::Result<Vec<TimestampedWeatherMap>, Box<dyn std::error::Error>> {
    let json = site.get_radar_meta()?;
    let response_items = parse_json(json)?;
    Ok(response_items
        .iter()
        .map(
            |item| -> std::result::Result<TimestampedWeatherMap, Box<dyn std::error::Error>> {
                Ok(TimestampedWeatherMap {
                    datetime: parse_datetime(item.valid.as_str())?,
                    weather_map: WeatherMap {
                        bbox: parse_bbox(item.bbox.as_str())?,
                        image: image::io::Reader::new(std::io::Cursor::new(
                            site.get_radar_image(item.path.as_str())?,
                        ))
                        .with_guessed_format()?
                        .decode()?
                        .to_rgba(),
                    },
                })
            },
        )
        .filter(|item| match item {
            Ok(v) => true,
            _ => false,
        })
        .map(|item| item.unwrap())
        .collect())
}

struct TimestampedWeatherMap {
    datetime: chrono::DateTime<chrono::FixedOffset>,
    weather_map: WeatherMap,
}

struct WeatherMap {
    //  datetime: chrono::DateTime<chrono::FixedOffset>,
    //  image_url: String,
    bbox: Bbox,
    image: image::RgbaImage,
}

#[cfg(test)]
mod tests {
    use super::parse_json;
    use super::RadarPrecipitationResponseItem;
    use chrono::offset::TimeZone;

    #[test]
    fn test_dummy() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_parse_json() {
        let s = "[{\"mode\":\"ANL\",\"path\":\"0.png\",\"date\":\"201911210245\",\"hhmm\":\"0245\",\"bbox\":\"44.67,12.1,47.42,17.44\",\"width\":\"800\",\"height\":\"600\",\"valid\":\"2019-11-21T02:45:00Z\"}]";
        assert_eq!(
            parse_json(s.to_string()).unwrap(),
            vec![RadarPrecipitationResponseItem {
                mode: "ANL".to_string(),
                path: "0.png".to_string(),
                date: "201911210245".to_string(),
                hhmm: "0245".to_string(),
                bbox: "44.67,12.1,47.42,17.44".to_string(),
                width: "800".to_string(),
                height: "600".to_string(),
                valid: "2019-11-21T02:45:00Z".to_string()
            }]
        );
    }

    #[test]
    fn test_load_radar() {
        let site = super::super::site::DummySiteDownloader::new();
        let mut result = super::load_radar(Box::new(site)).unwrap();
        assert_eq!(result.len(), 1);
        let first_timestamped_weather_map = result.pop().unwrap();
        assert_eq!(
            first_timestamped_weather_map.datetime,
            chrono::FixedOffset::east(0)
                .ymd(2019, 11, 21)
                .and_hms(02, 45, 00)
        )
    }
}
