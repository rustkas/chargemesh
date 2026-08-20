# ChargeMesh

**Universal interoperability runtime for EV charging infrastructure.**

ChargeMesh is an open-source software platform for connecting, testing, observing, diagnosing, and integrating electric vehicle charging infrastructure across different protocols, vendors, and generations of charging equipment.

It provides a unified software layer over protocols such as OCPP, ISO 15118, and OCPI.

```text
             EV Charging Infrastructure
                         │
          ┌──────────────┼──────────────┐
          │              │              │
        OCPP          ISO 15118        OCPI
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                ┌─────────────────┐
                │    ChargeMesh   │
                │                 │
                │     EV-IR       │
                │  Capabilities   │
                │  State Machine  │
                │  Diagnostics    │
                │  Simulation     │
                └────────┬────────┘
                         │
                  Unified API
                         │
              ┌──────────┼──────────┐
              ▼          ▼          ▼
            CSMS        EMS       SaaS
```

---

## Why ChargeMesh?

EV charging infrastructure is built from many independent systems:

- Charging stations from different vendors
- Different OCPP versions (1.6, 2.0.1, 2.1)
- ISO 15118 implementations
- Roaming networks (OCPI)
- Energy management systems
- Payment and authorization systems
- Vendor-specific APIs
- Different firmware versions and capabilities

Standards improve interoperability, but they do not eliminate implementation differences, protocol errors, vendor extensions, incompatible capabilities, and complex distributed-system failures.

ChargeMesh is designed to provide a common software layer that hides this complexity. It answers the fundamental question:

> **"Why is this charging session not working?"**

---

## Core Idea

ChargeMesh introduces a canonical **EV Charging Intermediate Representation (EV-IR)**.

Different protocols are translated into a common domain model:

```text
OCPP 1.6 ─────┐
OCPP 2.0.1 ───┤
OCPP 2.1 ─────┤
ISO 15118 ────┤───> EV-IR ───> Applications
OCPI ─────────┤
Vendor APIs ──┘
```

### EV-IR Entity Model

```text
┌────────────────────────────────────────────────────────┐
│                    ChargingNetwork                     │
│  + id: string                                          │
│  + name: string                                        │
│  + stations: ChargingStation[]                         │
│  + capabilities: Capabilities                          │
└───────────────────┬────────────────────────────────────┘
                    │
                    ▼
┌────────────────────────────────────────────────────────┐
│                    ChargingStation                     │
│  + id: StationId                                       │
│  + vendor: string                                      │
│  + model: string                                       │
│  + firmwareVersion: string                             │
│  + evses: EVSE[]                                       │
│  + capabilities: Capabilities                          │
│  + state: StationState                                 │
└───────────────────┬────────────────────────────────────┘
                    │
                    ▼
┌────────────────────────────────────────────────────────┐
│                    EVSE                                │
│  + id: EvseId                                          │
│  + connectors: Connector[]                             │
│  + maxPower: Power                                     │
│  + state: EVSEState                                    │
└───────────────────┬────────────────────────────────────┘
                    │
                    ▼
┌────────────────────────────────────────────────────────┐
│                  Connector                             │
│  + id: ConnectorId                                     │
│  + type: ConnectorType                                 │
│  + state: ConnectorState                               │
│  + currentSession?: ChargingSession                    │
│  + maxPower: Power                                     │
└───────────────────┬────────────────────────────────────┘
                    │
                    ▼
┌────────────────────────────────────────────────────────┐
│                   ChargingSession                      │
│  + id: SessionId                                       │
│  + stationId: StationId                                │
│  + evseId: EvseId                                      │
│  + connectorId: ConnectorId                            │
│  + state: SessionState                                 │
│  + startTime: Timestamp                                │
│  + endTime?: Timestamp                                 │
│  + energyConsumed: Energy                              │
└────────────────────────────────────────────────────────┘
```

### Supported Entities

| Entity             | Description                                       |
| ------------------ | ------------------------------------------------- |
| `ChargingNetwork`  | A network of charging stations (CPO fleet)        |
| `ChargingStation`  | Physical charging station (OCPP charge point)     |
| `EVSE`             | Electric Vehicle Supply Equipment (charging unit) |
| `Connector`        | Physical connector/plug (Type2, CCS, CHAdeMO)     |
| `Vehicle`          | Electric vehicle with battery and capabilities    |
| `ChargingSession`  | Charging transaction from start to stop           |
| `Transaction`      | Billing record with energy and cost               |
| `MeterValue`       | Energy measurement reading                        |
| `Tariff`           | Pricing model (flat, time-of-day, tiered)         |
| `Authorization`    | User/vehicle authentication                       |
| `Reservation`      | Connector reservation                             |
| `ChargingProfile`  | Smart charging schedule                           |
| `Capability`       | Supported features                                |
| `ChargingError`    | Normalized error (ChargeX MREC)                   |
| `Firmware`         | Firmware version and update status                |
| `EnergyConstraint` | Grid and energy limitations                       |

---

## Architecture

ChargeMesh is organized into several layers:

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                         APPLICATIONS                                        │
│         (CSMS, Energy Management, SaaS, Monitoring, Analytics)              │
├─────────────────────────────────────────────────────────────────────────────┤
│                    UNIFIED API (REST / gRPC / WebSocket)                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                    OBSERVABILITY & DIAGNOSTICS                              │
│         Metrics │ Logs │ Traces │ Events │ Analytics │ Root Cause           │
├─────────────────────────────────────────────────────────────────────────────┤
│                    CAPABILITY & STATE ENGINE                                │
│         Capability Discovery │ State Machines │ Rule Evaluation             │
├─────────────────────────────────────────────────────────────────────────────┤
│                    UNIVERSAL EV MODEL (EV-IR)                               │
│         Station │ EVSE │ Connector │ Session │ Transaction │ Meter          │
├─────────────────────────────────────────────────────────────────────────────┤
│                    PROTOCOL ADAPTER LAYER                                   │
├────────────┬────────────┬────────────┬────────────┬─────────────────────────┤
│  OCPP 1.6  │ OCPP 2.0.1 │ OCPP 2.1  │ ISO 15118  │        OCPI              │
└────────────┴────────────┴────────────┴────────────┴─────────────────────────┘
```

---

## Technology Stack

ChargeMesh uses **Rust** as its primary language for the core platform, with TypeScript for web-based tooling.

```text
┌───────────────────────────────────────────────────────────────────────────┐
│  FRONTEND           │  TypeScript / React / Next.js                       │
│                     │  WASM (Rust → WebAssembly)                          │
├───────────────────────────────────────────────────────────────────────────┤
│  API & COMMS        │  REST (Axum) │ gRPC (Tonic) │ WebSocket             │
├───────────────────────────────────────────────────────────────────────────┤
│  BACKEND (Rust)     │  Tokio (async runtime)                              │
│                     │  Serde (serialization)                              │
│                     │  Tracing (observability)                            │
│                     │  SQLx (database)                                    │
├───────────────────────────────────────────────────────────────────────────┤
│  DATA LAYER         │  PostgreSQL │ Redis │ NATS/Kafka │ Object Storage   │
├───────────────────────────────────────────────────────────────────────────┤
│  INFRASTRUCTURE     │  Docker │ Kubernetes │ Terraform                    │
│                     │  Prometheus │ Grafana │ ELK Stack │ Jaeger          │
└───────────────────────────────────────────────────────────────────────────┘
```

### Why Rust?

- **Memory safety** — Eliminates entire classes of bugs at compile time
- **Performance** — Zero-cost abstractions, near C++ performance
- **Concurrency** — Fearless concurrency with async/await
- **Reliability** — Type system prevents many errors at compile time
- **WASM** — First-class support for web-based tooling
- **Edge deployment** — Can run on Raspberry Pi and industrial controllers
- **Interoperability** — C-FFI, WASM, Node.js integration

---

## Commercial Model

ChargeMesh follows an open-core business model:

```text
┌────────────────────────────────────────────────────────────────────────────┐
│                         FREE / Open Source                                 │
│  • Protocol libraries (OCPP 1.6, 2.0.1, 2.1)                               │
│  • Universal EV Model (EV-IR)                                              │
│  • Simulator                                                               │
│  • Local diagnostics                                                       │
│  • SDK (Rust, TypeScript)                                                  │
│  • CLI + Web Inspector                                                     │
│  • Community support                                                       │
└────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────────┐
│                           PRO ($99/mo)                                     │
│  • Cloud monitoring                                                        │
│  • Fleet management                                                        │
│  • Advanced diagnostics                                                    │
│  • Protocol trace storage (30 days)                                        │
│  • Email/Slack alerts                                                      │
│  • Up to 100 stations                                                      │
│  • 10 users                                                                │
│  • 8/5 support                                                             │
└────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────────┐
│                      ENTERPRISE ($499/mo)                                  │
│  • Private deployment                                                      │
│  • SLA (99.9% uptime)                                                      │
│  • Custom integrations                                                     │
│  • Compliance (GDPR, HIPAA)                                                │
│  • Dedicated support                                                       │
│  • Large fleet management (unlimited)                                      │
│  • Unlimited users                                                         │
│  • 24/7 support                                                            │
│  • Custom features                                                         │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## Project Status

> **Early development — Phase 1 Complete**

ChargeMesh is currently in active research and development. The APIs, EV-IR model, protocol adapters, and architecture are expected to evolve significantly.

**Do not use the current versions for production charging infrastructure.**

### Progress

| Phase  | Component                                    | Status          |
| ------ | -------------------------------------------- | --------------- |
| P0     | Research & Specification                     | ✅ Complete     |
| P0     | Architecture documentation                   | ✅ Complete     |
| P0     | EV-IR specification                          | ✅ Complete     |
| P0     | Protocol model                               | ✅ Complete     |
| P0     | Capabilities model                           | ✅ Complete     |
| P0     | State machines specification                 | ✅ Complete     |
| P0     | Error taxonomy (ChargeX MREC)                | ✅ Complete     |
| **P1** | **Universal EV Model**                       | ✅ **Complete** |
| P1     | Core types (Power, Energy, Duration, etc.)   | ✅ Complete     |
| P1     | EV-IR entities (16 entities)                 | ✅ Complete     |
| P1     | State machines (Session, Station, Connector) | ✅ Complete     |
| P1     | Unit tests                                   | ✅ Complete     |
| P2     | OCPP 1.6                                     | 🚧 In Progress  |
| P2     | WebSocket capture                            | 📋 Planned      |
| P2     | Protocol logger                              | 📋 Planned      |
| P2     | CLI tool                                     | 📋 Planned      |
| P3     | Capability Engine                            | 📋 Planned      |
| P3     | Diagnostics Engine                           | 📋 Planned      |
| P4     | Simulator                                    | 📋 Planned      |
| P5     | Web Debugger (WASM)                          | 📋 Planned      |

### MVP v0.1 Scope

| Component                                    | Status         |
| -------------------------------------------- | -------------- |
| Core types (Power, Energy, Duration, etc.)   | ✅ Complete    |
| EV-IR model (16 entities)                    | ✅ Complete    |
| State machines (Session, Station, Connector) | ✅ Complete    |
| OCPP 1.6 (core messages)                     | 🚧 In Progress |
| WebSocket capture                            | 📋 Planned     |
| Protocol logger                              | 📋 Planned     |
| CLI tool                                     | 📋 Planned     |
| Basic Web debugger                           | 📋 Planned     |
| Human-readable diagnosis                     | 📋 Planned     |

---

## Example

A ChargeMesh application works with any charging station without knowing the underlying protocol:

```rust
use chargemesh_ir::prelude::*;
use chargemesh_core::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a charging station
    let mut station = ChargingStation::new(
        StationId::new("CP-001"),
        "ABB".to_string(),
        "Terra 54".to_string(),
    );

    // Add EVSE and connector
    let mut evse = EVSE::new(
        EvseId::new("EVSE-1"),
        StationId::new("CP-001"),
        Power::new(50000),
    );

    let connector = Connector::new(
        ConnectorId::new("CONN-1"),
        EvseId::new("EVSE-1"),
        ConnectorType::CCS,
        Power::new(50000),
    );
    evse.add_connector(connector);
    station.add_evse(evse);

    // Start a charging session
    let mut session = ChargingSession::new(
        StationId::new("CP-001"),
        EvseId::new("EVSE-1"),
        ConnectorId::new("CONN-1"),
    );

    // Use state machine
    let mut sm = SessionStateMachine::new();
    sm.start_authorization()?;
    sm.authorize()?;
    sm.start_charging()?;

    // Update session state
    session.transition_to(SessionState::Charging)?;

    // Add meter reading
    let meter = MeterValue::new(Energy::new(1000));
    session.add_meter_reading(meter);

    println!("Session ID: {}", session.id);
    println!("State: {:?}", session.state);

    Ok(())
}
```

The application does not need to know whether the underlying device uses OCPP 1.6, OCPP 2.0.1, OCPP 2.1, or a vendor-specific implementation.

---

## Repository Structure

```text
chargemesh/
├── Cargo.toml                                 # Workspace root
├── README.md                                  # This file
├── LICENSE                                    # License
├── docs/                                      # Documentation
│   ├── architecture.md                        # Architecture overview
│   ├── ev-ir.md                               # EV-IR specification
│   ├── protocol-model.md                      # Protocol model
│   ├── capabilities.md                        # Capabilities model
│   ├── state-machines.md                      # State machines
│   └── error-taxonomy.md                      # Error taxonomy (ChargeX MREC)
├── crates/
│   ├── chargemesh-core/                       # Core types & utilities
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── types.rs                       # Power, Energy, Duration, etc.
│   │       ├── error.rs                       # CoreError, CoreResult
│   │       ├── ident.rs                       # Id, StationId, EvseId, etc.
│   │       ├── time.rs                        # Timestamp, TimeRange
│   │       ├── crypto.rs                      # SHA256Hash, generate_token
│   │       └── config.rs                      # Configuration utilities
│   │
│   └── chargemesh-ir/                         # EV-IR model
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── network.rs                     # ChargingNetwork
│           ├── station.rs                     # ChargingStation
│           ├── evse.rs                        # EVSE
│           ├── connector.rs                   # Connector
│           ├── vehicle.rs                     # Vehicle
│           ├── session.rs                     # ChargingSession
│           ├── transaction.rs                 # Transaction
│           ├── meter.rs                       # Meter, MeterValue
│           ├── tariff.rs                      # Tariff
│           ├── authorization.rs               # Authorization
│           ├── reservation.rs                 # Reservation
│           ├── profile.rs                     # ChargingProfile
│           ├── capability.rs                  # Capabilities
│           ├── error.rs                       # ChargingError (ChargeX)
│           ├── firmware.rs                    # Firmware
│           ├── energy.rs                      # EnergyConstraint
│           └── state_machine/                 # State machines
│               ├── mod.rs
│               ├── session.rs                 # SessionStateMachine
│               ├── station.rs                 # StationStateMachine
│               └── connector.rs               # ConnectorStateMachine
├── tests/
│   ├── unit/
│   │   └── core_tests.rs                      # Core types tests
│   └── integration/
│       └── ir_tests.rs                        # EV-IR integration tests
├── apps/                                      # Applications (Phase 2+)
│   └── chargemesh-cli/                        # CLI (planned)
└── web/                                       # Web tools (Phase 5+)
    └── debugger/                              # Web debugger (planned)
```

---

## Development

### Requirements

- Rust stable toolchain (1.70+)
- Cargo
- Git

### Quick Start

```bash
# Clone the repository
git clone https://github.com/rustkas/chargemesh.git
cd chargemesh

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Run specific tests
cargo test -p chargemesh-ir
cargo test -p chargemesh-core
```

### Project Structure

```bash
# View project structure
tree -L 3

# Build documentation
cargo doc --workspace --open
```

---

## Design Principles

### Protocol Independence

Applications should depend on the ChargeMesh model rather than individual charging protocols.

### Capability-First Design

The system should describe what a device can actually do, rather than assuming capabilities from its protocol version.

### Explicit State Machines

Charging infrastructure is a distributed state-machine problem. State transitions should be explicit, observable, and testable.

### Diagnostics as a First-Class Capability

Protocol traces, state transitions, errors, and device events should be correlated into a single charging-session timeline.

### Observability by Design

Every component should expose metrics, logs, and traces. Nothing should be a black box.

### Edge-First Architecture

Core protocol processing should be capable of running close to the charging infrastructure (on-site gateways, Raspberry Pi, industrial PCs).

### Testability

The simulator should allow testing without physical equipment. Every scenario should be reproducible.

### Developer Experience

CLI and Web debugger should make troubleshooting intuitive. The learning curve should be gentle.

### Open Source Core

The fundamental interoperability layer remains open source. Commercial value is added through cloud services, advanced diagnostics, and enterprise features.

---

## Long-Term Vision

ChargeMesh aims to become a common software infrastructure layer for EV charging interoperability:

```text
                 Electric Vehicles
                        │
                        ▼
                 ISO 15118 / CAN
                        │
                        ▼
                 Charging Station
                        │
                        ▼
                      OCPP
                        │
                        ▼
                 ┌──────────────┐
                 │  ChargeMesh  │
                 │              │
                 │    EV-IR     │
                 │  Runtime     │
                 │  Diagnostics │
                 │  Simulator   │
                 └───────┬──────┘
                         │
              ┌──────────┼──────────┐
              ▼          ▼          ▼
             CSMS       OCPI       EMS
                                    │
                                   Grid
```

**The long-term objective is to make EV charging infrastructure programmable through a common runtime rather than through dozens of protocol- and vendor-specific integrations.**

---

## Roadmap

| Phase  | Component                  | Status         |
| ------ | -------------------------- | -------------- |
| **P0** | Research & Specification   | ✅ Complete    |
| **P1** | Universal EV Model (EV-IR) | ✅ Complete    |
| P2     | OCPP 1.6 Core              | 🚧 In Progress |
| P2     | OCPP Simulator             | 📋 Planned     |
| P2     | CLI Tool                   | 📋 Planned     |
| P3     | Capability Engine          | 📋 Planned     |
| P3     | Diagnostics Engine         | 📋 Planned     |
| P4     | Observability Platform     | 📋 Planned     |
| P5     | Web Debugger (WASM)        | 📋 Planned     |
| P6     | OCPI + Energy Integration  | 📋 Planned     |
| P7     | Cloud Platform             | 📋 Planned     |

---

## Contributing

Contributions, protocol implementations, interoperability reports, test cases, and real-world charging-session traces are welcome.

Please open an issue before implementing a major architectural change.

### Contribution Guidelines

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Areas Needing Help

- OCPP implementations (1.6, 2.0.1, 2.1)
- Additional vendor profiles
- Simulator scenarios
- Diagnostics rules
- Documentation
- Web tooling
- Testing

---

## License

TBD — likely Apache 2.0 with Commons Clause for commercial use.

---

## Acknowledgments

ChargeMesh builds upon the work of many standards bodies and open-source projects:

- [Open Charge Alliance](https://www.openchargealliance.org/) — OCPP
- [ISO 15118](https://www.iso.org/standard/55366.html) — V2G communication
- [OCPI](https://evroaming.org/) — Open Charge Point Interface
- [ChargeX Consortium](https://chargex.eu/) — Error taxonomy
- [Tokio](https://tokio.rs/) — Async runtime
- [Serde](https://serde.rs/) — Serialization

---

## Contact

- **GitHub Issues**: For bugs and feature requests
- **Discussions**: For questions and community support
- **Email**: [team@chargemesh.io](mailto:team@chargemesh.io)

---

**ChargeMesh: Making EV charging infrastructure programmable.**

```text
                    ┌─────────────────┐
                    │   ChargeMesh    │
                    │                 │
                    │  EV Charging    │
                    │ Interoperability│
                    │  for the Future │
                    └─────────────────┘
```

---

## Summary of Phase 1

### What was implemented:

| Component                                                                  | Status      |
| -------------------------------------------------------------------------- | ----------- |
| `chargemesh-core` crate                                                    | ✅ Complete |
| Core types (Power, Energy, Duration, Temperature, Percentage, Money)       | ✅ Complete |
| Identifiers (Id, StationId, EvseId, ConnectorId, SessionId, TransactionId) | ✅ Complete |
| Error handling (CoreError, CoreResult)                                     | ✅ Complete |
| Time utilities (Timestamp, TimeRange)                                      | ✅ Complete |
| Crypto utilities (SHA256Hash, generate_token)                              | ✅ Complete |
| `chargemesh-ir` crate                                                      | ✅ Complete |
| 16 EV-IR entities                                                          | ✅ Complete |
| State machines (Session, Station, Connector)                               | ✅ Complete |
| Unit tests                                                                 | ✅ Complete |
| Integration tests                                                          | ✅ Complete |
| Documentation (docs/)                                                      | ✅ Complete |
| README.md                                                                  | ✅ Updated  |

```

**Phase 1 is now fully complete!** 🚀
```
