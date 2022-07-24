use std::thread;

use arp_watch::{sniff::sniff, ui, App};
use log::debug;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    debug!("Logger initialised");
    let (app, app_tx) = App::new();
    thread::spawn(move || sniff("enp0s3", app_tx));
    ui::start_ui(app).unwrap();
}
