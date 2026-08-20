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
│  OCPP 1.6  │ OCPP 2.0.1 │ OCPP 2.1  │ ISO 15118  │        OCPI              │
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
│  DATA LAYER                   │  PostgreSQL │ Redis │ NATS/Kafka           │
├────────────────────────────────────────────────────────────────────────────┤
│  INFRASTRUCTURE               │  Docker │ Kubernetes │ Terraform           │
│                               │  Prometheus │ Grafana │ ELK Stack          │
└────────────────────────────────────────────────────────────────────────────┘
```

### Web Frontend (Emerge Core)

We use **`@emerge/core`** as the minimal reactive foundation:

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

#### Why Emerge Core?

| Aspect | Benefit |
|--------|---------|
| **Minimal Surface** | Only signals, computed, effects, and ownership |
| **Platform First** | Works with Web Platform, not against it |
| **No Magic** | Reactive graph and lifetime are explainable |
| **Lightweight** | No framework overhead, no Virtual DOM |
| **Composable** | Easy to build higher-level abstractions |
| **TypeScript** | Full type safety (optional for consumers) |

### Why Rust + WASM?

| Aspect | Benefit |
|--------|---------|
| **Protocol Analysis** | OCPP parsing runs natively in browser via WASM |
| **Performance** | Near-native speed for protocol processing |
| **Type Safety** | Full type safety across network boundaries |
| **Code Reuse** | Same Rust code for backend and frontend |
| **No JavaScript** | No runtime overhead for protocol logic |

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
│  • SDK (Rust)                                                              │
│  • CLI + Web Inspector (Emerge Core)                                       │
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

> **Early development — Phase 2 Complete**

ChargeMesh is currently in active research and development. The APIs, EV-IR model, protocol adapters, and architecture are expected to evolve significantly.

**Do not use the current versions for production charging infrastructure.**

### Progress

| Phase | Component | Status |
|-------|-----------|--------|
| P0 | Research & Specification | ✅ Complete |
| P0 | Architecture documentation | ✅ Complete |
| P0 | EV-IR specification | ✅ Complete |
| P0 | Protocol model | ✅ Complete |
| P0 | Capabilities model | ✅ Complete |
| P0 | State machines specification | ✅ Complete |
| P0 | Error taxonomy (ChargeX MREC) | ✅ Complete |
| **P1** | **Universal EV Model** | ✅ **Complete** |
| P1 | Core types (Power, Energy, Duration, etc.) | ✅ Complete |
| P1 | EV-IR entities (16 entities) | ✅ Complete |
| P1 | State machines (Session, Station, Connector) | ✅ Complete |
| P1 | Unit tests | ✅ Complete |
| **P2** | **OCPP 1.6 Core** | ✅ **Complete** |
| P2 | OCPP 1.6 messages (13 messages) | ✅ Complete |
| P2 | OCPP 1.6 parser | ✅ Complete |
| P2 | OCPP 1.6 client | ✅ Complete |
| P2 | OCPP 1.6 server | ✅ Complete |
| P2 | OCPP 1.6 state machine | ✅ Complete |
| P2 | WebSocket utilities | ✅ Complete |
| P2 | CLI tool (parse + capture) | ✅ Complete |
| P2 | Integration tests | ✅ Complete |
| P3 | OCPP 2.0.1 | 📋 Planned |
| P3 | OCPP 2.1 | 📋 Planned |
| P4 | Simulator | 📋 Planned |
| P5 | Diagnostics Engine | 📋 Planned |
| P6 | Observability Platform | 📋 Planned |
| P7 | Web Inspector (Emerge Core + WASM) | 📋 Planned |
| P8 | OCPI + Energy Integration | 📋 Planned |

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

# Show version
chargemesh version
```

---

## Example

### Backend: Connect to a charger and start a session

```rust
use chargemesh_ir::prelude::*;
use chargemesh_ocpp::v16::*;
use chargemesh_core::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to a charging station via OCPP 1.6
    let client = Ocpp16Client::connect("ws://charger:9000").await?;
    
    // Boot notification
    let boot_resp = client.boot_notification("ABB", "Terra 54").await?;
    println!("Boot status: {:?}", boot_resp.status);
    
    // Start a transaction
    let auth_resp = client.authorize("RFID-123").await?;
    let tx_resp = client.start_transaction(1, "RFID-123", 0).await?;
    println!("Transaction ID: {}", tx_resp.transaction_id);
    
    // Send meter values
    let meter = MeterValue {
        timestamp: chrono::Utc::now(),
        sampled_value: vec![SampledValue {
            value: "10.5".to_string(),
            context: Some(ReadingContext::SamplePeriodic),
            format: None,
            measurand: Some(Measurand::EnergyActiveImportRegister),
            unit: Some(UnitOfMeasure::kWh),
            location: None,
        }],
    };
    client.meter_values(1, Some(tx_resp.transaction_id), vec![meter]).await?;
    
    // Stop transaction
    client.stop_transaction(tx_resp.transaction_id, 1000, None, None).await?;
    
    client.close().await?;
    
    Ok(())
}
```

### Frontend: Web Inspector with Emerge Core

```typescript
// web/inspector/src/main.ts — Emerge Core + WASM

import {
  signal,
  computed,
  effect,
  createOwner,
  runWithOwner,
} from '@emerge/core';

// Import WASM module
import init, { parse_ocpp_message } from './wasm/chargemesh_wasm.js';

// Reactive state
const trace = signal<ParsedMessage[]>([]);
const selectedIndex = signal<number | null>(null);
const filter = signal<string>('');

// Computed: filtered messages
const filteredMessages = computed(() => {
  const query = filter.value.toLowerCase();
  return trace.value.filter(msg => 
    msg.action.toLowerCase().includes(query) ||
    msg.id_tag?.toLowerCase().includes(query)
  );
});

// Computed: session state machine
const sessionState = computed(() => {
  const sm = new Ocpp16Session();
  for (const msg of trace.value) {
    sm.process(msg);
  }
  return sm.state;
});

// Effect: update DOM when trace changes
const owner = createOwner();
runWithOwner(owner, () => {
  effect(() => {
    const messages = filteredMessages.value;
    const container = document.getElementById('timeline');
    if (container) {
      container.innerHTML = messages.map((msg, i) => `
        <div class="message ${i === selectedIndex.value ? 'selected' : ''}"
             data-index="${i}">
          <span class="time">${msg.timestamp}</span>
          <span class="direction">${msg.direction}</span>
          <span class="content">${msg.action}</span>
        </div>
      `).join('');
      
      // Attach event listeners
      container.querySelectorAll('.message').forEach(el => {
        el.addEventListener('click', () => {
          selectedIndex.value = parseInt(el.dataset.index!);
        });
      });
    }
  });
});

// Effect: update state machine display
runWithOwner(owner, () => {
  effect(() => {
    const state = sessionState.value;
    const el = document.getElementById('state');
    if (el) {
      el.textContent = `Current State: ${state}`;
    }
  });
});

// Load trace file
document.getElementById('load-btn')?.addEventListener('click', async () => {
  const file = (document.getElementById('file-input') as HTMLInputElement).files?.[0];
  if (!file) return;
  
  const content = await file.text();
  const lines = content.split('\n');
  const parsed: ParsedMessage[] = [];
  
  for (const line of lines) {
    if (line.trim()) {
      const msg = parse_ocpp_message(line);
      if (msg) parsed.push(msg);
    }
  }
  
  trace.value = parsed;
});

// Initialize WASM
await init();

// Cleanup on page unload
window.addEventListener('beforeunload', () => {
  owner.dispose();
});
```

### Custom Element for OCPP Inspector

```typescript
// web/inspector/src/components/ocpp-inspector.ts

import { signal, effect, createOwner, runWithOwner } from '@emerge/core';

class OcppInspector extends HTMLElement {
  private owner = createOwner();
  private trace = signal<ParsedMessage[]>([]);
  
  connectedCallback() {
    this.render();
    this.setupEffects();
  }
  
  disconnectedCallback() {
    this.owner.dispose();
  }
  
  private render() {
    this.innerHTML = `
      <div class="inspector">
        <header>
          <h1>⚡ OCPP Inspector</h1>
          <input type="file" id="load-trace" accept=".ocpp,.txt">
        </header>
        <main>
          <div id="timeline"></div>
          <div id="state"></div>
        </main>
      </div>
    `;
  }
  
  private setupEffects() {
    runWithOwner(this.owner, () => {
      effect(() => {
        const messages = this.trace.value;
        const container = this.querySelector('#timeline');
        if (container) {
          container.innerHTML = messages.map(m => 
            `<div>${m.timestamp} ${m.action}</div>`
          ).join('');
        }
      });
    });
    
    // File load handler
    this.querySelector('#load-trace')?.addEventListener('change', async (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (file) {
        const content = await file.text();
        // Parse and update trace
      }
    });
  }
}

customElements.define('ocpp-inspector', OcppInspector);
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
│   ├── chargemesh-ir/                         # EV-IR model
│   ├── chargemesh-ocpp/                       # OCPP Implementation
│   └── chargemesh-wasm/                       # WASM module for web
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs                         # Exports for Emerge
├── apps/
│   ├── chargemesh-cli/                        # Command-line interface
│   └── chargemesh-inspector/                  # Web Inspector
│       ├── Cargo.toml                         # Rust backend for inspector
│       └── src/
│           └── main.rs                        # Axum server
├── web/
│   └── inspector/                             # Frontend (Emerge Core)
│       ├── package.json
│       ├── tsconfig.json
│       ├── index.html
│       ├── src/
│       │   ├── main.ts                        # Application entry
│       │   ├── components/
│       │   │   ├── ocpp-inspector.ts          # Custom Element
│       │   │   └── state-machine.ts           # State visualization
│       │   └── wasm/                          # WASM bindings
│       │       └── index.ts
│       └── styles/
│           └── main.css
├── tests/
│   ├── unit/
│   ├── integration/
│   └── e2e/
└── examples/
```

---

## Development

### Requirements

- Rust stable toolchain (1.70+)
- Cargo
- Git
- Node.js 18+ (for web inspector)
- wasm-pack (for WASM compilation)

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

# Build WASM for web
wasm-pack build --target web crates/chargemesh-wasm

# Build web inspector
cd web/inspector
npm install
npm run build

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

Protocol traces, state transitions, errors, and device events should be correlated into a single charging-session timeline.

### Observability by Design

Every component should expose metrics, logs, and traces. Nothing should be a black box.

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
| P3 | OCPP 2.0.1 | 📋 Planned |
| P3 | OCPP 2.1 | 📋 Planned |
| P4 | Simulator | 📋 Planned |
| P5 | Diagnostics Engine | 📋 Planned |
| P6 | Observability Platform | 📋 Planned |
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
- Additional vendor profiles
- Simulator scenarios
- Diagnostics rules
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
- ✅ Unit and integration tests

### Phase 2 (Complete)
- ✅ `chargemesh-ocpp` — OCPP 1.6 implementation
- ✅ 13 OCPP 1.6 messages
- ✅ WebSocket client/server
- ✅ OCPP 1.6 parser and state machine
- ✅ CLI tool (`parse` and `capture`)
- ✅ Unit, integration, and E2E tests

### Next Steps
- Phase 3: OCPP 2.0.1 and OCPP 2.1
- Phase 4: Simulator
- Phase 5: Diagnostics Engine
- Phase 6: Observability Platform
- Phase 7: Web Inspector with Emerge Core + WASM
```