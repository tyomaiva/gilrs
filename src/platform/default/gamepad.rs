// Copyright 2016 GilRs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
#![allow(unused_variables)]

use gamepad::{self, Event, Status, PowerInfo, GamepadImplExt, MappingSource};
use ff::Error;
use mapping::{MappingData, MappingError};
use uuid::Uuid;

#[derive(Debug)]
pub struct Gilrs {
    not_observed: gamepad::Gamepad,
}

impl Gilrs {
    pub fn new() -> Self {
        warn!("Current platform is not supported, gamepad input will not work");
        Gilrs {
            not_observed: gamepad::Gamepad::from_inner_status(Gamepad::none(),
                                                              Status::NotObserved,
                                                              Default::default()),
        }
    }

    pub fn with_mappings(_sdl_mapping: &str) -> Self {
        Self::new()
    }

    pub fn next_event(&mut self) -> Option<(usize, Event)> {
        None
    }

    pub fn gamepad(&self, id: usize) -> &gamepad::Gamepad {
        &self.not_observed
    }

    pub fn gamepad_mut(&mut self, id: usize) -> &mut gamepad::Gamepad {
        &mut self.not_observed
    }

    /// Returns index greater than index of last connected gamepad.
    pub fn last_gamepad_hint(&self) -> usize {
        0
    }
}

#[derive(Debug)]
pub struct Gamepad {
    _priv: (),
}

impl Gamepad {
    fn none() -> Self {
        Gamepad { _priv: () }
    }

    pub fn name(&self) -> &str {
        ""
    }

    pub fn uuid(&self) -> Uuid {
        Uuid::nil()
    }

    pub fn power_info(&self) -> PowerInfo {
        PowerInfo::Unknown
    }

    pub fn mapping_source(&self) -> MappingSource {
        MappingSource::None
    }

    pub fn set_mapping(&mut self,
                       _mapping: &MappingData,
                       _strict: bool,
                       _name: Option<&str>)
                       -> Result<String, MappingError> {
        Err(MappingError::NotImplemented)
    }

    pub fn max_ff_effects(&self) -> usize {
        0
    }

    pub fn is_ff_supported(&self) -> bool {
        false
    }

    pub fn set_ff_gain(&mut self, gain: u16) -> Result<(), Error> {
        Err(Error::FfNotSupported)
    }
}

pub struct EventIterator<'a>(&'a mut Gilrs);

impl<'a> Iterator for EventIterator<'a> {
    type Item = (usize, Event);

    fn next(&mut self) -> Option<(usize, Event)> {
        None
    }
}

pub mod native_ev_codes {
    pub const BTN_SOUTH: u16 = 0;
    pub const BTN_EAST: u16 = 1;
    pub const BTN_C: u16 = 2;
    pub const BTN_NORTH: u16 = 3;
    pub const BTN_WEST: u16 = 4;
    pub const BTN_Z: u16 = 5;
    pub const BTN_LT: u16 = 6;
    pub const BTN_RT: u16 = 7;
    pub const BTN_LT2: u16 = 8;
    pub const BTN_RT2: u16 = 9;
    pub const BTN_SELECT: u16 = 10;
    pub const BTN_START: u16 = 11;
    pub const BTN_MODE: u16 = 12;
    pub const BTN_LTHUMB: u16 = 13;
    pub const BTN_RTHUMB: u16 = 14;

    pub const BTN_DPAD_UP: u16 = 15;
    pub const BTN_DPAD_DOWN: u16 = 16;
    pub const BTN_DPAD_LEFT: u16 = 17;
    pub const BTN_DPAD_RIGHT: u16 = 18;

    pub const AXIS_LSTICKX: u16 = 0;
    pub const AXIS_LSTICKY: u16 = 1;
    pub const AXIS_LEFTZ: u16 = 2;
    pub const AXIS_RSTICKX: u16 = 3;
    pub const AXIS_RSTICKY: u16 = 4;
    pub const AXIS_RIGHTZ: u16 = 5;
    pub const AXIS_DPADX: u16 = 6;
    pub const AXIS_DPADY: u16 = 7;
    pub const AXIS_RT: u16 = 8;
    pub const AXIS_LT: u16 = 9;
    pub const AXIS_RT2: u16 = 10;
    pub const AXIS_LT2: u16 = 11;
}
