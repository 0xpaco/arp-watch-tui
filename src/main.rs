use std::{env::args, thread};

use arp_watch::{sniff::sniff, ui, App};
use log::{debug, info};

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    debug!("Logger initialised");

    let (app, app_tx) = App::new();
    let args = args();
    info!("{}", args.len());
    if args.len() < 2 {
        thread::spawn(move || sniff("enp0s3", Some(app_tx)));
        ui::start_ui(app).unwrap();
    } else {
        sniff("enp0s3", None);
    }
}
