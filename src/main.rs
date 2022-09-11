use std::{env::args, thread};

use arp_watch::{sniff::sniff, ui, App};
use log::{debug, error, info};

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    debug!("Logger initialised");

    let mut args = args();
    if args.len() < 2 {
        error!("Usage: arpwatch <iface> [Ip mask]");
        return;
    }

    let ifacename = args.next().unwrap().clone();
    let (app, app_tx) = App::new();
    info!("{}", args.len());
    if args.len() < 3 {
        thread::spawn(move || sniff(ifacename.as_str(), Some(app_tx)));
        ui::start_ui(app).unwrap();
    } else {
        sniff(ifacename.as_str(), None);
    }
}
