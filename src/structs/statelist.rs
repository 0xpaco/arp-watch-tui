use tui::widgets::ListState;

use super::net::{Device, MacAddr};

pub struct StateList<T> {
    pub items: Vec<T>,
    pub state: ListState,
}

impl StateList<Device> {
    pub fn get_by_mac(&self, searched: &MacAddr) -> Option<&Device> {
        for device in self.items.iter() {
            if device.mac == *searched {
                return Some(device);
            }
        }
        None
    }

    pub fn get(&self, searched: &Device) -> Option<&Device> {
        for dev in self.items.iter() {
            if *dev == *searched {
                return Some(&dev);
            }
        }
        None
    }
}

impl<T> StateList<T> {
    pub fn new() -> StateList<T> {
        StateList {
            items: vec![],
            state: ListState::default(),
        }
    }

    pub fn next(&mut self) {
        // Making sure there is at least one item
        if self.items.len() == 0 {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i + 1 < self.items.len() {
                    i + 1
                } else {
                    i
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn prev(&mut self) {
        if self.items.len() == 0 {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => i.checked_sub(1).unwrap_or(0),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn select(&mut self, select: usize) {
        if select > self.items.len() {
            self.state.select(Some(self.items.len() - 1));
        } else {
            self.state.select(Some(select));
        }
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
