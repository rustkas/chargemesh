//! Session state machine

use super::*;
use crate::SessionState;

pub struct SessionStateMachine {
    state: SessionState,
}

impl SessionStateMachine {
    pub fn new() -> Self {
        Self {
            state: SessionState::Initializing,
        }
    }

    pub fn start_authorization(&mut self) -> Result<()> {
        self.transition(SessionState::Authorizing)
    }

    pub fn authorize(&mut self) -> Result<()> {
        self.transition(SessionState::Authorized)
    }

    pub fn start_charging(&mut self) -> Result<()> {
        self.transition(SessionState::Charging)
    }

    pub fn suspend(&mut self) -> Result<()> {
        self.transition(SessionState::Suspended)
    }

    pub fn resume(&mut self) -> Result<()> {
        self.transition(SessionState::Charging)
    }

    pub fn finish(&mut self) -> Result<()> {
        self.transition(SessionState::Finishing)
    }

    pub fn complete(&mut self) -> Result<()> {
        self.transition(SessionState::Completed)
    }

    pub fn fault(&mut self) -> Result<()> {
        self.transition(SessionState::Faulted)
    }
}

impl StateMachine for SessionStateMachine {
    type State = SessionState;

    fn state(&self) -> &SessionState {
        &self.state
    }

    fn can_transition(&self, new_state: &SessionState) -> bool {
        use SessionState::*;

        match (&self.state, new_state) {
            (Initializing, Authorizing) => true,
            (Initializing, Faulted) => true,
            (Authorizing, Authorized) => true,
            (Authorizing, Faulted) => true,
            (Authorized, Charging) => true,
            (Authorized, Faulted) => true,
            (Charging, Suspended) => true,
            (Charging, Finishing) => true,
            (Charging, Faulted) => true,
            (Suspended, Charging) => true,
            (Suspended, Finishing) => true,
            (Suspended, Faulted) => true,
            (Finishing, Completed) => true,
            (Finishing, Faulted) => true,
            (Completed, _) => false,
            (Faulted, _) => false,
            _ => false,
        }
    }

    fn transition(&mut self, new_state: SessionState) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_lifecycle() {
        let mut sm = SessionStateMachine::new();
        assert_eq!(*sm.state(), SessionState::Initializing);

        sm.start_authorization().unwrap();
        assert_eq!(*sm.state(), SessionState::Authorizing);

        sm.authorize().unwrap();
        assert_eq!(*sm.state(), SessionState::Authorized);

        sm.start_charging().unwrap();
        assert_eq!(*sm.state(), SessionState::Charging);

        sm.suspend().unwrap();
        assert_eq!(*sm.state(), SessionState::Suspended);

        sm.resume().unwrap();
        assert_eq!(*sm.state(), SessionState::Charging);

        sm.finish().unwrap();
        assert_eq!(*sm.state(), SessionState::Finishing);

        sm.complete().unwrap();
        assert_eq!(*sm.state(), SessionState::Completed);

        // Should fail - terminal state
        assert!(sm.start_charging().is_err());
    }

    #[test]
    fn test_fault_lifecycle() {
        let mut sm = SessionStateMachine::new();
        sm.start_authorization().unwrap();
        sm.authorize().unwrap();
        sm.start_charging().unwrap();
        sm.fault().unwrap();
        assert_eq!(*sm.state(), SessionState::Faulted);

        // Should fail - terminal state
        assert!(sm.complete().is_err());
    }
}