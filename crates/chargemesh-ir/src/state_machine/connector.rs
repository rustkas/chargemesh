//! Connector state machine

use super::*;
use crate::ConnectorState;

pub struct ConnectorStateMachine {
    state: ConnectorState,
}

impl ConnectorStateMachine {
    pub fn new() -> Self {
        Self {
            state: ConnectorState::Available,
        }
    }

    pub fn prepare(&mut self) -> Result<()> {
        self.transition(ConnectorState::Preparing)
    }

    pub fn start_charging(&mut self) -> Result<()> {
        self.transition(ConnectorState::Charging)
    }

    pub fn suspend(&mut self) -> Result<()> {
        self.transition(ConnectorState::Suspended)
    }

    pub fn resume(&mut self) -> Result<()> {
        self.transition(ConnectorState::Charging)
    }

    pub fn fault(&mut self) -> Result<()> {
        self.transition(ConnectorState::Faulted)
    }

    pub fn reset(&mut self) -> Result<()> {
        self.transition(ConnectorState::Available)
    }
}

impl StateMachine for ConnectorStateMachine {
    type State = ConnectorState;

    fn state(&self) -> &ConnectorState {
        &self.state
    }

    fn can_transition(&self, new_state: &ConnectorState) -> bool {
        use ConnectorState::*;

        match (&self.state, new_state) {
            (Available, Preparing) => true,
            (Available, Faulted) => true,
            (Preparing, Charging) => true,
            (Preparing, Available) => true,
            (Preparing, Faulted) => true,
            (Charging, Suspended) => true,
            (Charging, Available) => true,
            (Charging, Faulted) => true,
            (Suspended, Charging) => true,
            (Suspended, Available) => true,
            (Suspended, Faulted) => true,
            (Faulted, Available) => true,
            _ => false,
        }
    }

    fn transition(&mut self, new_state: ConnectorState) -> Result<()> {
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