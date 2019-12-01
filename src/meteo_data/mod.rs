extern crate askama;
use askama::Template;
mod px_to_precipitation;
mod radar;
mod site;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    is_raining: bool,
    description: String,
}

pub fn main(shared_app_state: super::SharedAppState) {
    let hundred_millis = std::time::Duration::from_millis(100);
    let mut count: u64 = 0;
    while shared_app_state
        .running
        .load(std::sync::atomic::Ordering::Relaxed)
    {
        std::thread::sleep(hundred_millis);

        if count % (10 * 60 * 5) == 0 {
            //If 5min elapsed
            let site = site::MeteoArsoGovSiDownloader::new();
            let result = radar::load_radar(Box::new(site));
            if result.is_ok() {
                let maps = result.unwrap();
                info!("Successfully loaded {} maps", maps.len());
                let newest_map = maps.iter().max_by_key(|x| x.datetime).unwrap();
                info!("Latest map: {}", newest_map.datetime);

                let lat = 46.0620872;
                let lon = 14.5428026;
                let px = newest_map.weather_map.get_pixel_at_coordinate(lat, lon);
                if px.is_some() {
                    let is_raining = px_to_precipitation::px_to_precipitation(px.unwrap()) > 0.1;

                    let tpl_data = IndexTemplate {
                        is_raining: is_raining,
                        description: format!(
                            "Precipitation: {}, Time: {}, Coords: {}, {}",
                            is_raining, newest_map.datetime, lat, lon
                        ),
                    };
                    *shared_app_state.index_html.write().unwrap() = tpl_data.render().unwrap();
                } else {
                    warn!("Error finding the pixel on the radar picture");
                }
            } else {
                warn!("Error loading radar picture: {}", result.unwrap_err());
            }
        }

        count += 1;
    }
}
