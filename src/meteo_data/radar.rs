use serde::{Deserialize, Serialize};
extern crate reqwest;

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
}
