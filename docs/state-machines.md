# State Machines in ChargeMesh

## 1. Purpose
This document defines the core state machines used by **ChargeMesh** to orchestrate charging sessions and device communication. Since different protocols (OCPP 1.6, 2.0.1, ISO 15118) have distinct state flows, **ChargeMesh** acts as a "Meta-State Machine" that reconciles them into a single, predictable lifecycle for the application layer (the EV-IR).

## 2. High-Level Charging Session States (EV-IR)
The EV-IR abstracts the charging lifecycle into a unified set of states:

1.  **Idle:** The connector is available and waiting for a vehicle or authorization.
2.  **Preparing:** The station is initializing the connection (handshake) with the EV or CSMS. *ISO 15118 focuses heavily on this initial V2G handshake.*
3.  **Authorizing:** The system is checking the user/vehicle credentials (RFID, Plug & Charge certificate).
4.  **Charging:** Active energy transfer.
    - *Smart Charging Sub-state:* The power is dynamically adjusted.
5.  **Stopping:** A stop command has been initiated (by user, vehicle, or CSMS).
6.  **Complete:** The session has ended normally.
7.  **Faulted:** An error has occurred requiring intervention.

```
┌─────────┐
│  IDLE   │
└────┬────┘
     │ Vehicle Connected / Authorization Requested
     ▼
┌─────────┐
│PREPARING│
└────┬────┘
     │ Authentication Success / Handshake Complete
     ▼
┌───────────┐
│AUTHORIZING│
└────┬──────┘
     │ Authorization Success
     ▼
┌─────────┐      ┌─────────────┐
│CHARGING │ ───► │  FAULTED    │ (Error detected)
└────┬────┘      └─────────────┘
     │ Stop Request / Session End
     ▼
┌─────────┐
│ STOPPING│
└────┬────┘
     │ Stop Complete
     ▼
┌─────────┐
│COMPLETE │
└─────────┘
```

## 3. Protocol-Specific Mappings

### 3.1. OCPP 1.6
- **State:** **BootNotification** → **Heartbeat** → **StatusNotification** (Available/Preparing/Charging/Faulted).
- **Transition:** Simple state machine with limited transactional integrity compared to OCPP 2.x.
- **Smart Charging:** Relies on `SetChargingProfile` without external constraint logic.
- **Flow:**
  1. Station sends `BootNotification`.
  2. CSMS responds with `Accepted`/`Rejected`.
  3. Station sends periodic `Heartbeat`.
  4. Station sends `StatusNotification` for each state change.
  5. Transaction: `StartTransaction` → `MeterValues` → `StopTransaction`.

### 3.2. ISO 15118
- **State:** Defined by the V2G (Vehicle-to-Grid) communication session states.
- **Sequence:** `SessionSetup` → `ServiceDiscovery` → `PaymentSelection` → `ChargeParameterDiscovery` → **`ChargingLoop`** → `SessionStop`.
- **Critical Complexity:** Requires state-aware message mutation and validation to ensure security and logical flow.
- **ChargeMesh Role:** ChargeMesh translates the complex V2G `ChargingLoop` (where parameters are continuously negotiated) into a stable "Charging" state in the EV-IR.

### 3.3. OCPP 2.0.1 / 2.1
- **State:** More granular device management states (Offline, Booted, Disabled, Enabled).
- **Transaction:** Supports multi-transaction sessions with enhanced state tracking.
- **Smart Charging:** Explicit state for applying `ChargingProfile` with "External Constraints" and "Local Generation".

## 4. Meta-State Machine Orchestration
The ChargeMesh orchestrator handles transitions based on protocol context:

```rust
impl MetaStateMachine {
    fn transition(&mut self, action: Action) -> Result<()> {
        match self.protocol {
            ProtocolType::OCPP16 => {
                // Map OCPP 1.6 specific events to EV-IR state transitions
            }
            ProtocolType::ISO15118 => {
                // Map V2G message sequence to EV-IR states
            }
            ProtocolType::OCPP21 => {
                // Use enhanced state capabilities
            }
        }
    }
}
```

## 5. Diagnostics & Error States
ChargeMesh defines a unique "Diagnostic" state where the station is not charging but is capable of reporting internal metrics (like `HighTemperature` or `PowerSwitchFailure`). This allows the system to differentiate between a station that is "Offline" and one that is "Operational but Faulted."

## 6. State Transition Validations
Each transition is validated against:
1. **Protocol Requirements:** Ensure the transition is allowed in the underlying protocol.
2. **Capability Constraints:** e.g., Smart Charging only allowed if capability exists.
3. **Security Policies:** Authorization must be valid before transitioning to `Charging`.
4. **Fault Detection:** Faulted state can be entered from any state (except Complete).

## 7. Event-Driven Architecture
State transitions emit events that can be:
- Logged for auditing.
- Triggered for monitoring alerts.
- Used to update dashboards in real-time.
- Sent to external systems via webhooks.

## 8. State Persistence
The current state of each station, EVSE, connector, and session is:
- Stored in Redis for fast access.
- Written to PostgreSQL for historical tracking.
- Replicated across nodes for high availability (in distributed deployments).
