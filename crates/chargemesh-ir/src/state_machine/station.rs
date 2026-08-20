//! Station state machine

use super::*;
use crate::StationState;

pub struct StationStateMachine {
    state: StationState,
}

impl StationStateMachine {
    pub fn new() -> Self {
        Self {
            state: StationState::Offline,
        }
    }

    pub fn boot(&mut self) -> Result<()> {
        self.transition(StationState::Booted)
    }

    pub fn enable(&mut self) -> Result<()> {
        self.transition(StationState::Enabled)
    }

    pub fn disable(&mut self) -> Result<()> {
        self.transition(StationState::Disabled)
    }

    pub fn fault(&mut self) -> Result<()> {
        self.transition(StationState::Faulted)
    }

    pub fn maintenance(&mut self) -> Result<()> {
        self.transition(StationState::Maintenance)
    }

    pub fn updating(&mut self) -> Result<()> {
        self.transition(StationState::Updating)
    }
}

impl StateMachine for StationStateMachine {
    type State = StationState;

    fn state(&self) -> &StationState {
        &self.state
    }

    fn can_transition(&self, new_state: &StationState) -> bool {
        use StationState::*;

        match (&self.state, new_state) {
            (Offline, Booted) => true,
            (Offline, Maintenance) => true,
            (Booted, Enabled) => true,
            (Booted, Disabled) => true,
            (Booted, Faulted) => true,
            (Booted, Maintenance) => true,
            (Enabled, Disabled) => true,
            (Enabled, Faulted) => true,
            (Enabled, Maintenance) => true,
            (Enabled, Updating) => true,
            (Disabled, Enabled) => true,
            (Disabled, Faulted) => true,
            (Disabled, Maintenance) => true,
            (Disabled, Booted) => true,
            (Faulted, Booted) => true,
            (Faulted, Enabled) => true,
            (Faulted, Disabled) => true,
            (Faulted, Maintenance) => true,
            (Maintenance, Booted) => true,
            (Maintenance, Enabled) => true,
            (Maintenance, Disabled) => true,
            (Updating, Booted) => true,
            (Updating, Enabled) => true,
            (Updating, Faulted) => true,
            _ => false,
        }
    }

    fn transition(&mut self, new_state: StationState) -> Result<()> {
        if self.can_transition(&new_state) {
            self.state = new_state;
            Ok(())
        } else {
            Err(CoreError::InvalidState(
                format!("Cannot transition from {:?} to {:?}", self.state, new_state)
            ))
        }
    }
}