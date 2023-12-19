use log::LevelFilter::{Debug, Error, Info, Warn};
pub const APP_ID: &str = "org.gtk-rs.termirust";
pub const APP_NAME: &str = "IndexRust";
pub const APP_WINDOW_WIDTH: i32 = 350;
pub const APP_WINDOW_HEIGHT: i32 = 500;
pub const MIN_SCROLL_WINDOW_HEIGHT: i32 = 400;
pub const INDEX_FOLDER: &str = "/home/ekla/Documents/";
pub fn set_log_level(level: &str) {
    let log_level = match level {
        "debug" => Debug,
        "info" => Info,
        "warn" => Warn,
        "error" => Error,
        _ => Error,
    };

    env_logger::Builder::from_default_env()
        .filter_level(log_level)
        .init();
}
