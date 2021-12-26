use crate::randstat;
use anyhow::Error;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct State {
    pub status: randstat::RandStat,
}

impl State {
    pub fn new(init_vec: &[randstat::StatInit]) -> Result<Self, Error> {
        Ok(Self {
            status: randstat::RandStat::new(init_vec)?,
        })
    }
}

pub type SharedState = Arc<RwLock<State>>;

pub fn wrap_shared_state(state: State) -> SharedState {
    Arc::new(RwLock::new(state))
}

#[derive(Debug)]
#[repr(u8)]
pub enum Event {
    None = 0,
    Disconnect = 1,
    Delay = 2,
    DropMessage = 3,
}

impl From<u8> for Event {
    fn from(i: u8) -> Self {
        match i {
            0x00 => Event::None,
            0x01 => Event::Disconnect,
            0x02 => Event::Delay,
            0x03 => Event::DropMessage,
            _ => panic!("unknown Event {}", i),
        }
    }
}
