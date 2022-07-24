use std::sync::mpsc::{self, Receiver, Sender};

pub mod sniff;
pub mod structs;
pub mod ui;

use crate::structs::arp::ArpPacket;
use crate::structs::statelist::StateList;

pub enum InputMode {
    EditMode,
    NormalMode,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Popup {
    GotoCommand,
    None,
}

pub struct App {
    // TODO: Change string for a made struct
    pub list: StateList<String>,
    pub rx: Receiver<ArpPacket>,
    scroll: usize,
    pub mode: InputMode,
    pub popup: Popup,
    pub input: String,
}

impl App {
    pub fn new() -> (App, Sender<ArpPacket>) {
        let (tx, rx) = mpsc::channel();
        (
            App {
                list: StateList::new(),
                rx,
                scroll: 0,
                mode: InputMode::NormalMode,
                popup: Popup::None,
                input: String::new(),
            },
            tx,
        )
    }

    pub fn scroll(&self) -> usize {
        self.scroll
    }

    pub fn set_scroll(&mut self, to: usize) {
        if to > self.list.items.len() {
            self.scroll = self.list.items.len() - 1;
        } else {
            self.scroll = to
        }
    }
}
