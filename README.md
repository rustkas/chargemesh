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

EV-IR provides common representations for:

- Charging stations
- EVSEs
- Connectors
- Vehicles
- Charging sessions
- Transactions
- Meter values
- Tariffs
- Authorization
- Reservations
- Charging profiles
- Capabilities
- Errors
- Energy constraints

This allows applications to work with any charging infrastructure without knowing the underlying protocol.

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
┌─────────────────────────────────────────────────────────────────────────────┐
│  FRONTEND           │  TypeScript / React / Next.js                         │
│                     │  WASM (Rust → WebAssembly)                            │
├─────────────────────────────────────────────────────────────────────────────┤
│  API & COMMS        │  REST (Axum) │ gRPC (Tonic) │ WebSocket               │
├─────────────────────────────────────────────────────────────────────────────┤
│  BACKEND (Rust)     │  Tokio (async runtime)                                │
│                     │  Serde (serialization)                                │
│                     │  Tracing (observability)                              │
│                     │  SQLx (database)                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│  DATA LAYER         │  PostgreSQL │ Redis │ NATS/Kafka │ Object Storage     │
├─────────────────────────────────────────────────────────────────────────────┤
│  INFRASTRUCTURE     │  Docker │ Kubernetes │ Terraform                      │
│                     │  Prometheus │ Grafana │ ELK Stack │ Jaeger            │
└─────────────────────────────────────────────────────────────────────────────┘
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

> **Early development — MVP in progress**

ChargeMesh is currently in active research and development. The APIs, EV-IR model, protocol adapters, and architecture are expected to evolve significantly.

**Do not use the current versions for production charging infrastructure.**

### MVP v0.1 Scope

| Component | Status |
|-----------|--------|
| OCPP 1.6 (core messages) | ✅ Complete |
| WebSocket capture | ✅ Complete |
| EV-IR model | ✅ Complete |
| Station/EVSE/Connector models | ✅ Complete |
| Session state machine | ✅ Complete |
| Protocol logger | ✅ Complete |
| Error taxonomy (ChargeX MREC) | ✅ Complete |
| Simulator (basic scenarios) | ✅ Complete |
| CLI tool | ✅ Complete |
| Basic Web debugger | ✅ Complete |
| Human-readable diagnosis | ✅ Complete |
| Root cause analysis (basic) | ✅ Complete |
| Recommendations | ✅ Complete |

---

## Example

A ChargeMesh application works with any charging station without knowing the underlying protocol:

```rust
use chargemesh::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to a charging station
    let station = ChargeMesh::connect(StationConfig {
        id: "CP-001".to_string(),
        protocol: Protocol::Ocpp16,
        endpoint: "ws://charger:9000".to_string(),
    }).await?;
    
    // Discover capabilities
    let caps = station.capabilities().await?;
    println!("Smart Charging: {}", caps.smart_charging);
    println!("ISO 15118: {}", caps.iso15118);
    println!("V2G: {}", caps.v2g);
    
    // Start charging
    let session = station.start_charging(StartConfig {
        authorization: Authorization::Rfid("RFID-123".to_string()),
        max_power: Some(22000),
        smart_charging: true,
    }).await?;
    
    // Monitor session
    while session.is_active().await? {
        let energy = session.energy().await?;
        let power = session.current_power().await?;
        println!("⚡ {} kW | {} kWh delivered", 
            power.as_kw(), 
            energy.as_kwh()
        );
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
    
    Ok(())
}
```

The application does not need to know whether the underlying device uses OCPP 1.6, OCPP 2.0.1, OCPP 2.1, or a vendor-specific implementation.

---

## Repository Structure

```text
chargemesh/
├── Cargo.toml                         # Workspace root
├── README.md                          # This file
├── crates/
│   ├── chargemesh-core/               # Core types & utilities
│   ├── chargemesh-ir/                 # EV-IR model
│   ├── chargemesh-ocpp/               # OCPP 1.6, 2.0.1, 2.1
│   ├── chargemesh-ocpi/               # OCPI integration
│   ├── chargemesh-iso15118/           # ISO 15118 / V2G
│   ├── chargemesh-capability/         # Capability engine
│   ├── chargemesh-diagnostics/        # Diagnostics engine
│   ├── chargemesh-observability/      # Observability platform
│   ├── chargemesh-simulator/          # Full simulator
│   └── chargemesh-integration/        # Energy & Smart Charging
├── apps/
│   ├── chargemesh-cli/                # Command-line interface
│   └── chargemesh-inspector/          # Desktop Inspector
├── web/
│   └── debugger/                      # Web debugger with WASM
├── examples/                          # Example applications
├── tests/                             # Integration tests
└── docs/                              # Documentation
```

---

## Development

### Requirements

- Rust stable toolchain (1.70+)
- Cargo
- Git
- Node.js 18+ (for web tooling)
- PostgreSQL 15+ (optional, for cloud features)
- Redis 7+ (optional, for caching)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/chargemesh/chargemesh.git
cd chargemesh

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Install the CLI
cargo install --path apps/chargemesh-cli

# Try it out
chargemesh inspect --help
```

### Web Debugger

```bash
# Build WASM module
wasm-pack build --target web crates/chargemesh-wasm

# Build web debugger
cd web/debugger
npm install
npm run build

# Serve locally
npm run serve
```

### Docker

```bash
# Build the Docker image
docker build -t chargemesh .

# Run the inspector
docker run --rm chargemesh inspect --connect ws://charger:9000
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

| Phase | Component | Status |
|-------|-----------|--------|
| 1 | EV-IR specification | ✅ Complete |
| 1 | OCPP 1.6 core | ✅ Complete |
| 1 | Charging session state machine | ✅ Complete |
| 1 | Capability model | ✅ Complete |
| 1 | OCPP simulator | ✅ Complete |
| 1 | Protocol trace format | ✅ Complete |
| 1 | Diagnostics engine | ✅ Complete |
| 2 | Web-based Inspector (WASM) | 🚧 In Progress |
| 2 | OCPP 2.0.1 | 📋 Planned |
| 3 | OCPI | 📋 Planned |
| 3 | ISO 15118 | 📋 Planned |
| 3 | OCPP 2.1 | 📋 Planned |
| 4 | Cloud observability | 📋 Planned |
| 4 | Energy-management integrations | 📋 Planned |
| 4 | V2G / V2X support | 📋 Planned |

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

- Protocol implementations (OCPP 2.0.1, 2.1, ISO 15118)
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
                    ┌──────────────────┐
                    │   ChargeMesh     │
                    │                  │
                    │  EV Charging     │
                    │  Interoperability│
                    │  for the Future  │
                    └──────────────────┘
```
