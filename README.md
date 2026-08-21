# Обновлённый `README.md` (Phase 8)

```markdown
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
┌─────────────────────────────────────────────────────────┐
│                    ChargingNetwork                      │
│  + id: string                                          │
│  + name: string                                        │
│  + stations: ChargingStation[]                         │
│  + capabilities: Capabilities                          │
└───────────────────┬─────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────────────────────┐
│                    ChargingStation                      │
│  + id: StationId                                       │
│  + vendor: string                                      │
│  + model: string                                       │
│  + firmwareVersion: string                             │
│  + evses: EVSE[]                                       │
│  + capabilities: Capabilities                          │
│  + state: StationState                                 │
└───────────────────┬─────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────────────────────┐
│                    EVSE                                 │
│  + id: EvseId                                          │
│  + connectors: Connector[]                              │
│  + maxPower: Power                                     │
│  + state: EVSEState                                    │
└───────────────────┬─────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────────────────────┐
│                  Connector                              │
│  + id: ConnectorId                                     │
│  + type: ConnectorType                                 │
│  + state: ConnectorState                               │
│  + currentSession?: ChargingSession                    │
│  + maxPower: Power                                     │
└───────────────────┬─────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────────────────────┐
│                   ChargingSession                       │
│  + id: SessionId                                       │
│  + stationId: StationId                                │
│  + evseId: EvseId                                      │
│  + connectorId: ConnectorId                            │
│  + state: SessionState                                 │
│  + startTime: Timestamp                                │
│  + endTime?: Timestamp                                 │
│  + energyConsumed: Energy                              │
└─────────────────────────────────────────────────────────┘
```

### Supported Entities

| Entity | Description |
|--------|-------------|
| `ChargingNetwork` | A network of charging stations (CPO fleet) |
| `ChargingStation` | Physical charging station (OCPP charge point) |
| `EVSE` | Electric Vehicle Supply Equipment (charging unit) |
| `Connector` | Physical connector/plug (Type2, CCS, CHAdeMO) |
| `Vehicle` | Electric vehicle with battery and capabilities |
| `ChargingSession` | Charging transaction from start to stop |
| `Transaction` | Billing record with energy and cost |
| `MeterValue` | Energy measurement reading |
| `Tariff` | Pricing model (flat, time-of-day, tiered) |
| `Authorization` | User/vehicle authentication |
| `Reservation` | Connector reservation |
| `ChargingProfile` | Smart charging schedule |
| `Capabilities` | Supported features |
| `ChargingError` | Normalized error (ChargeX MREC) |
| `Firmware` | Firmware version and update status |
| `EnergyConstraint` | Grid and energy limitations |

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
│  OCPP 1.6  │ OCPP 2.0.1 │ OCPP 2.1   │ ISO 15118  │        OCPI             │
└────────────┴────────────┴────────────┴────────────┴─────────────────────────┘
```

---

## Technology Stack

ChargeMesh uses **Rust** as its primary language for the core platform, with **Emerge Core** providing the reactive foundation for web tooling.

### Core Platform (Rust)

```text
┌────────────────────────────────────────────────────────────────────────────┐
│  BACKEND (Rust)               │  Tokio (async runtime)                     │
│                               │  Serde (serialization)                     │
│                               │  Tracing (observability)                   │
│                               │  SQLx (database)                           │
├────────────────────────────────────────────────────────────────────────────┤
│  PROTOCOLS                    │  OCPP 1.6, 2.0.1, 2.1                      │
│                               │  ISO 15118 / V2G                           │
│                               │  OCPI (roaming)                            │
├────────────────────────────────────────────────────────────────────────────┤
│  CAPABILITY ENGINE            │  Multi-factor capability detection         │
│                               │  Protocol │ Vendor │ Firmware │ Runtime    │
│                               │  Rule-based evaluation                     │
│                               │  Vendor profiles (ABB, Siemens)            │
├────────────────────────────────────────────────────────────────────────────┤
│  DIAGNOSTICS ENGINE           │  Timeline collection                       │
│                               │  Pattern, Sequence, Performance, Security  │
│                               │  Root Cause Analysis with confidence       │
│                               │  Report generation (JSON, HTML)            │
├────────────────────────────────────────────────────────────────────────────┤
│  OBSERVABILITY PLATFORM       │  Metrics (Counters, Gauges, Histograms)    │
│                               │  Structured Logging with rich metadata     │
│                               │  Event Bus for real-time events            │
│                               │  Correlation Tracing (Device→Root Cause)   │
│                               │  Terminal Dashboard with widgets           │
├────────────────────────────────────────────────────────────────────────────┤
│  INTEGRATION LAYER            │  OCPI Client/Server (roaming)              │
│                               │  EMS, DER, BESS integration                │
│                               │  V2G (Vehicle-to-Grid)                     │
│                               │  Smart Charging Optimizer                  │
│                               │  Grid constraints & renewable energy       │
├────────────────────────────────────────────────────────────────────────────┤
│  CLOUD PLATFORM               │  REST API (Axum)                           │
│                               │  Tenant management                         │
│                               │  Billing & subscriptions                   │
│                               │  Analytics & reporting                     │
│                               │  JWT authentication                        │
├────────────────────────────────────────────────────────────────────────────┤
│  SIMULATOR                    │  EV │ EVSE │ CSMS │ OCPI │ Grid            │
│                               │  Fault injection │ Scenarios               │
│                               │  Normal │ Network Failure │ V2G            │
├────────────────────────────────────────────────────────────────────────────┤
│  DATA LAYER                   │  PostgreSQL │ Redis │ NATS/Kafka           │
├────────────────────────────────────────────────────────────────────────────┤
│  INFRASTRUCTURE               │  Docker │ Kubernetes │ Terraform           │
│                               │  Prometheus │ Grafana │ ELK Stack          │
└────────────────────────────────────────────────────────────────────────────┘
```

### Web Frontend (Emerge Core)

```text
┌────────────────────────────────────────────────────────────────────────────┐
│  FRONTEND (Emerge Core)       │  @emerge/core (signals, computed, effects) │
│                               │  • signal() — reactive state               │
│                               │  • computed() — lazy derived values        │
│                               │  • effect() — scheduled side effects       │
│                               │  • createOwner() — lifetime management     │
│                               │  • Custom Elements for UI components       │
│                               │  • WASM for protocol analysis              │
│                               │  • No Virtual DOM — direct DOM updates     │
└────────────────────────────────────────────────────────────────────────────┘
```

### Why Rust + Emerge Core?

| Aspect | Benefit |
|--------|---------|
| **Protocol Analysis** | OCPP parsing and capability detection run natively in browser via WASM |
| **Performance** | Near-native speed for protocol processing and capability evaluation |
| **Type Safety** | Full type safety across network boundaries |
| **Code Reuse** | Same Rust code for backend and frontend |
| **No JavaScript** | No runtime overhead for protocol logic |
| **Capability Engine** | Multi-factor detection with rule-based evaluation |
| **Diagnostics Engine** | Timeline analysis and root cause detection |
| **Observability** | Full metrics, logs, events, and correlations |
| **Integration** | OCPI roaming, Energy Management, V2G, Smart Charging |
| **Cloud Platform** | Multi-tenant, billing, analytics |
| **Simulator** | Full ecosystem simulation without physical hardware |

---

## Commercial Model

ChargeMesh follows an open-core business model:

```text
┌────────────────────────────────────────────────────────────────────────────┐
│                         FREE / Open Source                                 │
│  • Protocol libraries (OCPP 1.6, 2.0.1, 2.1)                               │
│  • Universal EV Model (EV-IR)                                              │
│  • Capability Engine                                                       │
│  • Diagnostics Engine (local)                                              │
│  • Observability Platform (local)                                          │
│  • Integration Layer (OCPI, Energy, V2G, Smart Charging)                   │
│  • Simulator (EV, EVSE, CSMS, OCPI, Grid)                                  │
│  • SDK (Rust)                                                              │
│  • CLI with 10 commands                                                    │
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

> **Early development — Phase 8 Complete**

ChargeMesh is currently in active research and development. The APIs, EV-IR model, protocol adapters, and architecture are expected to evolve significantly.

**Do not use the current versions for production charging infrastructure.**

### Progress

| Phase | Component | Status |
|-------|-----------|--------|
| P0 | Research & Specification | ✅ Complete |
| **P1** | **Universal EV Model** | ✅ **Complete** |
| **P2** | **OCPP 1.6 Core** | ✅ **Complete** |
| **P3** | **Capability Engine** | ✅ **Complete** |
| **P4** | **Simulator** | ✅ **Complete** |
| **P5** | **Diagnostics Engine** | ✅ **Complete** |
| **P6** | **Observability Platform** | ✅ **Complete** |
| **P7** | **OCPI + Energy Integration** | ✅ **Complete** |
| **P8** | **Cloud Platform** | ✅ **Complete** |
| P8 | REST API (10 endpoints) | ✅ Complete |
| P8 | Tenant Management | ✅ Complete |
| P8 | Billing & Subscriptions | ✅ Complete |
| P8 | Analytics Engine | ✅ Complete |
| P8 | JWT Authentication | ✅ Complete |
| P8 | CLI command `cloud` | ✅ Complete |
| P9 | Web Inspector (Emerge Core + WASM) | 📋 Planned |

---

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/rustkas/chargemesh.git
cd chargemesh

# Build all crates
cargo build --workspace

# Install the CLI
cargo install --path apps/chargemesh-cli
```

### Quick Start

```bash
# Parse an OCPP trace file
chargemesh parse --file trace.ocpp --verbose

# Capture traffic from a charger
chargemesh capture --url ws://charger:9000 --output captured.ocpp --duration 30

# Analyze capabilities of a charging station
chargemesh capability --config station.json --format human --verbose

# Run a simulation
chargemesh simulate --target charger --scenario normal

# Diagnose a trace file
chargemesh diagnose --file trace.ocpp --verbose

# Generate HTML report
chargemesh diagnose --file trace.ocpp --format html --output report.html

# Observe a station (live monitoring)
chargemesh observe --station-id CP-001 --follow

# OCPI: List locations from a roaming partner
chargemesh ocpi --url https://cpo.example.com/ocpi --token abc123 --country DE --party CPO locations

# Energy: Check EMS status
chargemesh energy --config ems.json status

# Smart Charging: Run optimization
chargemesh energy --config ems.json optimize

# Cloud: Login and check status
chargemesh cloud login --url https://api.chargemesh.cloud --token <your-token>
chargemesh cloud status

# Cloud: List stations and sessions
chargemesh cloud stations
chargemesh cloud sessions

# Cloud: Get analytics
chargemesh cloud analytics

# Cloud: Subscription info
chargemesh cloud subscription

# List available scenarios
chargemesh simulate --list-scenarios

# Show version
chargemesh version
```

---

## Examples

### 1. Parse OCPP Trace

```bash
chargemesh parse --file trace.ocpp --verbose
```

```text
📂 Parsing file: trace.ocpp
📊 Parsed 42 messages

📝 Message Timeline
  12:01:03 ⬅️  Call(BootNotification)
  12:01:05 ➡️  CallResult(1)
  12:03:21 ⬅️  Call(Authorize)
  12:03:22 ➡️  CallResult(3)
  12:03:25 ⬅️  Call(StartTransaction)
  12:03:25 ➡️  CallResult(4)
  12:03:26 ⬅️  Call(StatusNotification)
```

### 2. OCPI Roaming — Get Locations

```bash
chargemesh ocpi --url https://cpo.example.com/ocpi --token abc123 --country DE --party CPO locations
```

```json
{
  "data": [
    {
      "id": "LOC-001",
      "name": "Berlin Central",
      "address": "123 Main St",
      "city": "Berlin",
      "country": "DE",
      "coordinates": { "latitude": 52.52, "longitude": 13.40 },
      "evses": [
        {
          "id": "EVSE-1",
          "status": "Available",
          "capabilities": ["SmartChargingCapable", "PlugAndChargeCapable"],
          "connectors": [
            {
              "id": "CONN-1",
              "connector_type": "CCS2",
              "max_power": 50000
            }
          ]
        }
      ]
    }
  ]
}
```

### 3. Cloud Platform — Login and List Stations

```bash
# Login
chargemesh cloud login --url https://api.chargemesh.cloud --token eyJhbGciOiJIUzI1NiIs...

# Check status
chargemesh cloud status
```

```text
🔭 ChargeMesh Cloud Status
  ✅ Cloud is online
  Version: 0.1.0
  Environment: production
```

```bash
# List stations
chargemesh cloud stations
```

```text
📡 Listing stations...
  📡 Found 3 stations:
    • CP-001 (Terra 54) - online
    • CP-002 (Terra 54) - charging
    • CP-003 (VersiCharge) - online
```

### 4. Smart Charging Optimization

```bash
chargemesh energy --config ems.json optimize
```

```text
⚡ Running smart charging optimization...

  Session: SESS-001
    Schedule:
      12:00 - 13:30 | 11.0 kW | 16.5 kWh
    Total Energy: 16.5 kWh
    Total Cost: €2.48
    Carbon Emissions: 6.6 kg CO2
    Target: MinimizeCost
```

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
│   ├── chargemesh-ir/                         # EV-IR model
│   ├── chargemesh-ocpp/                       # OCPP Implementation
│   ├── chargemesh-capability/                 # Capability Engine
│   ├── chargemesh-simulator/                  # Simulator
│   ├── chargemesh-diagnostics/                # Diagnostics Engine
│   ├── chargemesh-observability/              # Observability Platform
│   ├── chargemesh-integration/                # OCPI + Energy Integration
│   └── chargemesh-cloud/                      # ⭐ Cloud Platform
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── core/                          # Core platform
│           │   └── platform.rs                # Main platform
│           ├── api/                           # REST API
│           │   ├── mod.rs
│           │   └── rest.rs                    # API endpoints
│           ├── billing/                       # Billing
│           │   ├── mod.rs
│           │   └── subscription.rs            # Subscriptions
│           ├── tenant/                        # Multi-tenant
│           │   ├── mod.rs
│           │   └── manager.rs                 # Tenant management
│           └── analytics/                     # Analytics
│               ├── mod.rs
│               └── aggregator.rs              # Data aggregation
├── apps/
│   └── chargemesh-cli/                        # Command-line interface
│       ├── Cargo.toml
│       └── src/
│           └── main.rs                        # 10 commands
├── web/
│   └── inspector/                             # Web Inspector (Phase 9+)
├── tests/
│   ├── unit/
│   ├── integration/
│   │   ├── ir_tests.rs
│   │   ├── ocpp_tests.rs
│   │   ├── capability_tests.rs
│   │   ├── simulator_tests.rs
│   │   ├── diagnostics_tests.rs
│   │   ├── observability_tests.rs
│   │   ├── integration_tests.rs
│   │   └── cloud_tests.rs                     # ⭐ Cloud tests
│   └── e2e/
│       ├── ocpp_e2e_tests.rs
│       ├── capability_e2e_tests.rs
│       ├── simulator_e2e_tests.rs
│       ├── diagnostics_e2e_tests.rs
│       ├── observability_e2e_tests.rs
│       ├── integration_e2e_tests.rs
│       └── cloud_e2e_tests.rs                 # ⭐ Cloud E2E tests
└── examples/
    ├── basic_connect.rs
    ├── diagnose_trace.rs
    ├── capability_analysis.rs
    ├── run_simulation.rs
    └── ocpi_integration.rs
```

---

## Development

### Requirements

- Rust stable toolchain (1.70+)
- Cargo
- Git
- PostgreSQL 15+ (for cloud platform)
- Redis 7+ (for caching)
- Node.js 18+ (for web inspector, Phase 9+)
- wasm-pack (for WASM compilation, Phase 9+)

### Build

```bash
# Clone the repository
git clone https://github.com/rustkas/chargemesh.git
cd chargemesh

# Build all crates
cargo build --workspace

# Build in release mode
cargo build --release --workspace

# Run tests
cargo test --workspace

# Run specific tests
cargo test -p chargemesh-ir
cargo test -p chargemesh-ocpp
cargo test -p chargemesh-capability
cargo test -p chargemesh-simulator
cargo test -p chargemesh-diagnostics
cargo test -p chargemesh-observability
cargo test -p chargemesh-integration
cargo test -p chargemesh-cloud

# View project structure
tree -L 3

# Build documentation
cargo doc --workspace --open
```

### CLI Usage

```bash
# Parse a trace file
chargemesh parse --file examples/trace.ocpp --verbose

# Capture traffic
chargemesh capture --url ws://charger:9000 --output captured.ocpp --duration 60

# Analyze capabilities
chargemesh capability --config station.json --format human --verbose

# Run a simulation
chargemesh simulate --target charger --scenario normal --duration 30

# Diagnose a trace
chargemesh diagnose --file trace.ocpp --verbose

# Generate HTML report
chargemesh diagnose --file trace.ocpp --format html --output report.html

# Observe a station (live monitoring)
chargemesh observe --station-id CP-001 --follow

# OCPI: List locations
chargemesh ocpi --url https://cpo.example.com/ocpi --token abc123 --country DE --party CPO locations

# OCPI: List sessions
chargemesh ocpi --url https://cpo.example.com/ocpi --token abc123 --country DE --party CPO sessions

# Energy: Check EMS status
chargemesh energy --config ems.json status

# Energy: Run optimization
chargemesh energy --config ems.json optimize

# Cloud: Login
chargemesh cloud login --url https://api.chargemesh.cloud --token <your-token>

# Cloud: List stations
chargemesh cloud stations

# Cloud: Get analytics
chargemesh cloud analytics

# Cloud: Subscription info
chargemesh cloud subscription

# List scenarios
chargemesh simulate --list-scenarios

# Show version
chargemesh version
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

Protocol traces, state transitions, errors, and device events should be correlated into a single charging-session timeline. Root cause analysis should identify the actual problem, not just the symptom.

### Observability by Design

Every component should expose metrics, logs, traces, and events. Nothing should be a black box. Correlations should connect device → protocol → session → error → root cause.

### Edge-First Architecture

Core protocol processing should be capable of running close to the charging infrastructure (on-site gateways, Raspberry Pi, industrial PCs).

### Testability

The simulator should allow testing without physical equipment. Every scenario should be reproducible.

### Developer Experience

CLI and Web Inspector should make troubleshooting intuitive. The learning curve should be gentle.

### Open Source Core

The fundamental interoperability layer remains open source. Commercial value is added through cloud services, advanced diagnostics, and enterprise features.

### Minimal Foundations

Using `@emerge/core` as the reactive foundation allows us to build exactly what we need — no more, no less. We own the rendering layer, the component model, and the tooling.

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
| **P0** | Research & Specification | ✅ Complete |
| **P1** | Universal EV Model (EV-IR) | ✅ Complete |
| **P2** | OCPP 1.6 Core | ✅ Complete |
| **P3** | Capability Engine | ✅ Complete |
| **P4** | Simulator | ✅ Complete |
| **P5** | Diagnostics Engine | ✅ Complete |
| **P6** | Observability Platform | ✅ Complete |
| **P7** | OCPI + Energy Integration | ✅ Complete |
| **P8** | Cloud Platform | ✅ Complete |
| P9 | Web Inspector (Emerge Core + WASM) | 📋 Planned |

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

- OCPP 2.0.1 implementation
- OCPP 2.1 implementation
- Additional vendor profiles (Alfen, Tritium, etc.)
- Simulator scenarios
- Diagnostics rules and patterns
- Documentation
- Web Inspector (Emerge Core + WASM)
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
- [Emerge Core](https://github.com/rustkas/emerge-core) — Reactive foundation

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

## Summary

### Phase 1 (Complete)
- ✅ `chargemesh-core` — Core types, identifiers, error handling
- ✅ `chargemesh-ir` — 16 EV-IR entities, state machines

### Phase 2 (Complete)
- ✅ `chargemesh-ocpp` — OCPP 1.6 implementation
- ✅ 13 OCPP 1.6 messages, WebSocket, parser, state machine

### Phase 3 (Complete)
- ✅ `chargemesh-capability` — Capability Engine
- ✅ Multi-factor detection, rule engine, vendor profiles

### Phase 4 (Complete)
- ✅ `chargemesh-simulator` — Full simulation environment
- ✅ EV, EVSE, CSMS, OCPI, Grid simulators, fault injection, scenarios

### Phase 5 (Complete)
- ✅ `chargemesh-diagnostics` — Diagnostics Engine
- ✅ Timeline Collector, 4 analyzers, Root Cause Analysis, Report Generator

### Phase 6 (Complete)
- ✅ `chargemesh-observability` — Observability Platform
- ✅ Metrics, Structured Logging, Event Bus, Correlation Tracing, Dashboard

### Phase 7 (Complete)
- ✅ `chargemesh-integration` — OCPI + Energy Integration
- ✅ OCPI Client/Server, EMS, V2G, Smart Charging Optimizer

### Phase 8 (Complete)
- ✅ `chargemesh-cloud` — Cloud Platform
- ✅ REST API (10 endpoints)
- ✅ Tenant Management
- ✅ Billing & Subscriptions
- ✅ Analytics Engine
- ✅ JWT Authentication
- ✅ CLI command `cloud` (6 subcommands)

### Next Steps
- Phase 9: Web Inspector with Emerge Core + WASM
```