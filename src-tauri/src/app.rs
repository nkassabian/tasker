use std::sync::{Mutex, OnceLock};

use duckdb::Connection;

pub struct App {
    pub connection: Connection,
}

impl App {
    fn new() -> Self {
        let connection = Connection::open_in_memory().unwrap();
        Self { connection }
    }
}

static APP: OnceLock<Mutex<App>> = OnceLock::new();

pub fn get_app() -> &'static Mutex<App> {
    let app = APP.get_or_init(|| Mutex::new(App::new()));

    return app;
}

pub fn use_app() {
    let app = get_app().lock().unwrap();
    // Use the app
}
