//! ChargeMesh IR — Universal EV Charging Intermediate Representation
//!
//! The canonical data model for all EV charging entities.
//! Protocol-independent representation of stations, EVSEs, connectors,
//! sessions, transactions, and all related concepts.

pub mod station;
pub mod evse;
pub mod connector;
pub mod vehicle;
pub mod session;
pub mod transaction;
pub mod meter;
pub mod tariff;
pub mod authorization;
pub mod reservation;
pub mod profile;
pub mod capability;
pub mod error;
pub mod firmware;
pub mod energy;
pub mod state_machine;
pub mod geo;

pub use station::*;
pub use evse::*;
pub use connector::*;
pub use vehicle::*;
pub use session::*;
pub use transaction::*;
pub use meter::*;
pub use tariff::*;
pub use authorization::*;
pub use reservation::*;
pub use profile::*;
pub use capability::*;
pub use error::*;
pub use firmware::*;
pub use energy::*;
pub use state_machine::*;
pub use geo::*;

use chargemesh_core::{Id, Timestamp, Power, Energy, StationId};
use serde::{Deserialize, Serialize};

/// IR version
pub const IR_VERSION: &str = "0.1.0";
