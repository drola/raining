#[macro_use]
extern crate log;
mod meteo_data;
mod web_server;

#[derive(Clone)]
pub struct SharedAppState {
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    index_html: std::sync::Arc<std::sync::RwLock<String>>,
}

impl SharedAppState {
    fn new() -> SharedAppState {
        SharedAppState {
            running: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)),
            index_html: std::sync::Arc::new(std::sync::RwLock::new(String::from("Initial state"))),
        }
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let shared_app_state = SharedAppState::new();
    let shared_app_state_clone = shared_app_state.clone();

    let t1 = std::thread::Builder::new()
        .name("web_server".to_string())
        .spawn(move || {
            web_server::main(shared_app_state_clone);
        })?;

    let shared_app_state_clone = shared_app_state.clone();
    let t2 = std::thread::Builder::new()
        .name("radar_data".to_string())
        .spawn(move || {
            meteo_data::main(shared_app_state_clone);
        })?;

    t1.join().expect("web_server thread has panicked");
    t2.join().expect("radar_data thread has panicked");
    Ok(())
}
