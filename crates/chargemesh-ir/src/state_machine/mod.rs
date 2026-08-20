//! State machine definitions

pub mod session;
pub mod station;
pub mod connector;

pub use session::*;
pub use station::*;
pub use connector::*;

use chargemesh_core::*;

/// Trait for state machines
pub trait StateMachine {
    type State: Clone + Serialize + for<'de> Deserialize<'de> + PartialEq + Eq;

    fn transition(&mut self, new_state: Self::State) -> Result<()>;
    fn state(&self) -> &Self::State;
    fn can_transition(&self, new_state: &Self::State) -> bool;
}