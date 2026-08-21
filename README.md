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
│  FRONTEND (Emerge Core)        │  @emerge/core (signals, computed, effects)│
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
│  • Simulator (EV, EVSE, CSMS, OCPI, Grid)                                  │
│  • SDK (Rust)                                                              │
│  • CLI with 7 commands                                                     │
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

> **Early development — Phase 6 Complete**

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
| P6 | Metrics (Counters, Gauges, Histograms) | ✅ Complete |
| P6 | Structured Logging | ✅ Complete |
| P6 | Event Bus | ✅ Complete |
| P6 | Correlation Tracing (Device→Root Cause) | ✅ Complete |
| P6 | Terminal Dashboard (5 widgets) | ✅ Complete |
| P6 | CLI command `observe` | ✅ Complete |
| P7 | Web Inspector (Emerge Core + WASM) | 📋 Planned |
| P8 | OCPI + Energy Integration | 📋 Planned |
| P9 | Cloud Platform | 📋 Planned |

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

# Observe a session
chargemesh observe --session-id SESS-123 --verbose

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

### 2. Observe a Station (Live Monitoring)

```bash
chargemesh observe --station-id CP-001 --follow
```

```text
╔═════════════════════════════════════════════════════════════════╗
║                          ChargeMesh                             ║
║                    Observability Platform                       ║
╠═════════════════════════════════════════════════════════════════╣
║  Updated: 2024-01-15 12:03:26                                   ║
╠═════════════════════════════════════════════════════════════════╣
║                                                                 ║
║  📊 OVERVIEW                                                    ║
║                                                                 ║
║    Metrics:       47  Logs:      123  Events:       89          ║
║    Traces:         3  Correlations:     12                      ║
║                                                                 ║
║  🏭 STATIONS                                                    ║
║                                                                 ║
║    Total:    12431  Online:    11984  Charging:     4832        ║
║    Errors:      183                                             ║
║                                                                 ║
║  ❌ ERRORS                                                      ║
║                                                                 ║
║    Protocol:     47  ISO 15118:     21                          ║
║    Network:      62  Hardware:      53                          ║
║                                                                 ║
║  📡 PROTOCOLS                                                   ║
║                                                                 ║
║    OCPP 1.6:     45  OCPP 2.0.1:    78                          ║
║    OCPP 2.1:     12  ISO 15118:     21                          ║
║                                                                 ║
║  🔗 CORRELATIONS                                                ║
║                                                                 ║
║    Device → Session:   12                                       ║
║    Session → Error:     8                                       ║
║    Error → Root Cause:  4                                       ║
║    Total:              24                                       ║
║                                                                 ║
╚═════════════════════════════════════════════════════════════════╝
```

### 3. Diagnose a Trace

```bash
chargemesh diagnose --file trace.ocpp --verbose
```

```text
═══════════════════════════════════════════════════════════
  🔍 DIAGNOSTIC REPORT
═══════════════════════════════════════════════════════════

📊 SUMMARY
  Found 1 root cause(s) with 3 total findings.
  1 critical issue(s) detected.

  Top root causes:
  1. Certificate Validation Failure (confidence: 94%)

🔍 ROOT CAUSES

  1. Certificate Validation Failure
     Confidence: 94%
     ISO 15118 certificate validation failed

     Possible causes:
       • Certificate has expired (probability: 40%)
         💡 Renew certificate and update system time
       • Certificate trust chain is invalid (probability: 30%)
         💡 Verify and update trust chain configuration
       • System time is incorrect (probability: 20%)
         💡 Synchronize system time with NTP
       • SECC certificate mismatch (probability: 10%)
         💡 Update SECC certificate configuration

💡 RECOMMENDATIONS
  • Check certificate validity
    Verify certificate is not expired and trust chain is valid
```

### 4. Run Simulation

```bash
chargemesh simulate --target charger --scenario network-failure --verbose
```

```text
🎮 Running simulation: charger
  Protocol: ocpp-1.6
  Station: ABB Terra 54
  Scenario: network-failure

🔄 Running scenario...
Step 1: ConnectEV
🔌 Connecting EV...
Step 2: Authorize { token: "RFID-123" }
🔑 Authorizing with token: RFID-123
Step 3: StartCharging
⚡ Starting charging...
Step 4: InjectFault { fault: NetworkDisconnect }
💥 Injecting fault: NetworkDisconnect
Step 5: WaitFor { condition: NetworkReconnected }
⏳ Waiting for: NetworkReconnected
Step 6: StopCharging
⏹️ Stopping charging...

✅ Simulation completed successfully
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
│   └── chargemesh-observability/              # ⭐ Observability Platform
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── platform/                      # Core platform
│           │   ├── observability.rs           # Main coordinator
│           │   └── config.rs                  # Configuration
│           ├── metrics/                       # Metrics system
│           │   ├── registry.rs                # Metric registry
│           │   └── collector.rs               # Metrics collector
│           ├── logging/                       # Structured logging
│           │   └── logger.rs                  # Logger implementation
│           ├── events/                        # Event bus
│           │   └── bus.rs                     # Event bus implementation
│           ├── correlations/                  # Correlation tracing
│           │   ├── tracer.rs                  # Correlation tracer
│           │   └── graph.rs                   # Correlation graph
│           └── dashboard/                     # Dashboard renderer
│               ├── renderer.rs                # Dashboard renderer
│               └── widgets.rs                 # Widget implementations
├── apps/
│   └── chargemesh-cli/                        # Command-line interface
│       ├── Cargo.toml
│       └── src/
│           └── main.rs                        # Parse, Capture, Capability, Simulate, Diagnose, Observe
├── web/
│   └── inspector/                             # Web Inspector (Phase 7+)
├── tests/
│   ├── unit/
│   ├── integration/
│   │   ├── ir_tests.rs
│   │   ├── ocpp_tests.rs
│   │   ├── capability_tests.rs
│   │   ├── simulator_tests.rs
│   │   ├── diagnostics_tests.rs
│   │   └── observability_tests.rs             # ⭐ Observability tests
│   └── e2e/
│       ├── ocpp_e2e_tests.rs
│       ├── capability_e2e_tests.rs
│       ├── simulator_e2e_tests.rs
│       ├── diagnostics_e2e_tests.rs
│       └── observability_e2e_tests.rs         # ⭐ Observability E2E tests
└── examples/
    ├── basic_connect.rs
    ├── diagnose_trace.rs
    ├── capability_analysis.rs
    └── run_simulation.rs
```

---

## Development

### Requirements

- Rust stable toolchain (1.70+)
- Cargo
- Git
- Node.js 18+ (for web inspector, Phase 7+)
- wasm-pack (for WASM compilation, Phase 7+)

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

# View project structure
tree -L 3

# Build documentation
cargo doc --workspace --open
```

### CLI Usage

```bash
# Parse a trace file
chargemesh parse --file examples/trace.ocpp --verbose

# Parse with JSON output
chargemesh parse --file trace.ocpp --format json

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

# Observe a session
chargemesh observe --session-id SESS-123 --verbose

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
| P7 | Web Inspector (Emerge Core + WASM) | 📋 Planned |
| P8 | OCPI + Energy Integration | 📋 Planned |
| P9 | Cloud Platform | 📋 Planned |

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
                    ┌──────────────────┐
                    │   ChargeMesh     │
                    │                  │
                    │  EV Charging     │
                    │  Interoperability│
                    │  for the Future  │
                    └──────────────────┘
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
- ✅ Metrics (Counters, Gauges, Histograms)
- ✅ Structured Logging with rich metadata
- ✅ Event Bus for real-time events
- ✅ Correlation Tracing (Device → Protocol → Session → Error → Root Cause)
- ✅ Terminal Dashboard with 5 widgets
- ✅ CLI command `observe`

### Next Steps
- Phase 7: Web Inspector with Emerge Core + WASM# Обновлённый `README.md` (Phase 6)
