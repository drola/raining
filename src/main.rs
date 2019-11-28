#[macro_use]
extern crate log;
mod meteo_data;
mod px_to_precipitation;
mod web_server;

#[derive(Clone)]
pub struct SharedAppState {
    index_html: std::sync::Arc<std::sync::RwLock<String>>,
}

impl SharedAppState {
    fn new() -> SharedAppState {
        SharedAppState {
            index_html: std::sync::Arc::new(std::sync::RwLock::new(String::from("Initial state"))),
        }
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let shared_app_state = SharedAppState::new();
    let shared_app_state_clone = shared_app_state.clone();

    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let running_clone = running.clone();
    let _ = std::thread::Builder::new()
        .name("web_server".to_string())
        .spawn(move || {
            web_server::main(shared_app_state_clone);
            running_clone.store(false, std::sync::atomic::Ordering::Relaxed);
            warn!("Web server thread exiting...");
        })?;

    let two_seconds = std::time::Duration::from_secs(2);
    let mut count: u32 = 1;
    while running.load(std::sync::atomic::Ordering::Relaxed) {
        std::thread::sleep(two_seconds);
        *shared_app_state.index_html.write().unwrap() = format!("Count: {}", count);

        //TODO: This part should actually fetch new weather data and update index_html with the new weather

        count += 1;
    }

    Ok(())
}
