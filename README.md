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

---

## Commercial Model

ChargeMesh follows an open-core business model:

```text
┌────────────────────────────────────────────────────────────────────────────┐
│                         FREE / Open Source                                 │
│  • Protocol libraries (OCPP 1.6, 2.0.1, 2.1)                               │
│  • Universal EV Model (EV-IR)                                              │
│  • Capability Engine                                                       │
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

> **Early development — Phase 3 Complete**

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
| **P2** | **OCPP 1.6 Core** | ✅ **Complete** |
| P2 | OCPP 1.6 messages (13 messages) | ✅ Complete |
| P2 | OCPP 1.6 parser | ✅ Complete |
| P2 | OCPP 1.6 client | ✅ Complete |
| P2 | OCPP 1.6 server | ✅ Complete |
| P2 | OCPP 1.6 state machine | ✅ Complete |
| P2 | WebSocket utilities | ✅ Complete |
| P2 | CLI tool (parse + capture) | ✅ Complete |
| **P3** | **Capability Engine** | ✅ **Complete** |
| P3 | Capability detection (protocol, vendor, firmware, runtime) | ✅ Complete |
| P3 | CapabilityRegistry with defaults | ✅ Complete |
| P3 | Rule engine with conditions and actions | ✅ Complete |
| P3 | Vendor profiles (ABB, Siemens) | ✅ Complete |
| P3 | CLI command `capability` | ✅ Complete |
| P4 | Simulator | 📋 Planned |
| P5 | Diagnostics Engine | 📋 Planned |
| P6 | Observability Platform | 📋 Planned |
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

# Show version
chargemesh version
```

### Capability Configuration Example

```json
{
  "station_id": "CP-001",
  "vendor": {
    "name": "ABB",
    "id": "ABB",
    "known_models": ["Terra 54", "Terra HP"]
  },
  "model": "Terra 54",
  "firmware": {
    "version": "2.1.0",
    "build_date": "2024-01-15"
  },
  "protocol": {
    "name": "OCPP",
    "version": "2.0.1",
    "transport": "WebSocket",
    "security_profile": "TLS"
  },
  "configuration": {
    "certificate_enabled": true,
    "max_power": 50000
  },
  "runtime": {
    "is_online": true,
    "is_booted": true,
    "active_sessions": 0,
    "temperature": 35.0,
    "load_percentage": 50
  },
  "hardware_version": "2.0",
  "serial_number": "SN12345"
}
```

### Capability Output Example

```text
═══════════════════════════════════════════════════════════
  🔧 CAPABILITY REPORT
═══════════════════════════════════════════════════════════

📊 CAPABILITIES
  • BasicCharging               ✅ Supported
  • ConfigurationManagement     ✅ Supported
  • OCPP1_6                     ✅ Supported
  • OCPP2_0_1                   ✅ Supported
  • OCPP2_1                     ❌ Not supported
  • PlugAndCharge               ✅ Supported
  • RemoteDiagnostics           ✅ Supported
  • RemoteFirmwareUpdate        ✅ Supported
  • RemoteReset                 ✅ Supported
  • Reservation                 ✅ Supported
  • RFIDAuthorization           ✅ Supported
  • SmartCharging               ✅ Supported

💡 SUMMARY
  Total: 12
  Supported: 10
  Limited: 2

═══════════════════════════════════════════════════════════
```

---

## Example

### Backend: Analyze station capabilities

```rust
use chargemesh_capability::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = HashMap::new();
    config.insert("certificate_enabled".to_string(), serde_json::json!(true));

    let context = CapabilityContext {
        station_id: "CP-001".to_string(),
        protocol: ProtocolInfo {
            name: ProtocolName::OCPP,
            version: "2.0.1".to_string(),
            transport: "WebSocket".to_string(),
            security_profile: "TLS".to_string(),
        },
        vendor: VendorInfo {
            name: "ABB".to_string(),
            id: Some("ABB".to_string()),
            known_models: vec!["Terra 54".to_string()],
        },
        firmware: FirmwareInfo {
            version: "2.1.0".to_string(),
            build_date: Some("2024-01-15".to_string()),
            checksum: None,
            compatibility: vec![],
        },
        configuration: config,
        runtime: RuntimeState {
            is_online: true,
            is_booted: true,
            active_sessions: 0,
            total_energy_delivered: 1000,
            uptime_seconds: 3600,
            temperature: Some(35.0),
            load_percentage: Some(50),
        },
        model: "Terra 54".to_string(),
        hardware_version: Some("2.0".to_string()),
        serial_number: Some("SN12345".to_string()),
    };

    let engine = CapabilityEngine::new();
    let capabilities = engine.determine_capabilities(&context).await?;

    // Check if station supports smart charging
    if capabilities.is_supported(&CapabilityType::SmartCharging) {
        println!("✅ Station supports Smart Charging");
    }

    // Get capability details as JSON
    let json = capabilities.to_json();
    println!("{}", serde_json::to_string_pretty(&json)?);

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

// Import WASM modules
import init, { parse_ocpp_message, analyze_capabilities } from './wasm/chargemesh_wasm.js';

// Reactive state
const trace = signal<ParsedMessage[]>([]);
const capabilities = signal<CapabilitySet | null>(null);
const selectedIndex = signal<number | null>(null);

// Computed: filtered messages
const filteredMessages = computed(() => {
  return trace.value;
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
    }
  });
});

// Effect: update capability display
runWithOwner(owner, () => {
  effect(() => {
    const caps = capabilities.value;
    const container = document.getElementById('capabilities');
    if (container && caps) {
      container.innerHTML = Object.entries(caps).map(([key, value]) => `
        <div class="capability ${value.supported ? 'supported' : 'unsupported'}">
          <span class="name">${key}</span>
          <span class="status">${value.supported ? '✅' : '❌'}</span>
        </div>
      `).join('');
    }
  });
});

// Load trace file and analyze capabilities
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

  // Analyze capabilities from the trace
  const caps = analyze_capabilities(parsed);
  capabilities.value = caps;
});

// Initialize WASM
await init();

// Cleanup on page unload
window.addEventListener('beforeunload', () => {
  owner.dispose();
});
```

### Custom Element for OCPP Inspector with Capability Display

```typescript
// web/inspector/src/components/ocpp-inspector.ts

import { signal, effect, createOwner, runWithOwner } from '@emerge/core';

class OcppInspector extends HTMLElement {
  private owner = createOwner();
  private trace = signal<ParsedMessage[]>([]);
  private capabilities = signal<CapabilitySet | null>(null);

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
          <div class="actions">
            <input type="file" id="load-trace" accept=".ocpp,.txt">
            <button id="analyze-btn">🔍 Analyze Capabilities</button>
          </div>
        </header>
        <main>
          <div class="panels">
            <div class="panel" id="timeline-panel">
              <h2>📊 Timeline</h2>
              <div id="timeline"></div>
            </div>
            <div class="panel" id="capability-panel">
              <h2>🔧 Capabilities</h2>
              <div id="capabilities"></div>
            </div>
          </div>
        </main>
      </div>
    `;

    // File load handler
    this.querySelector('#load-trace')?.addEventListener('change', async (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (file) {
        const content = await file.text();
        // Parse and update trace
      }
    });

    // Analyze button handler
    this.querySelector('#analyze-btn')?.addEventListener('click', async () => {
      // Use WASM to analyze capabilities from current trace
    });
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

    runWithOwner(this.owner, () => {
      effect(() => {
        const caps = this.capabilities.value;
        const container = this.querySelector('#capabilities');
        if (container && caps) {
          container.innerHTML = Object.entries(caps).map(([key, value]) =>
            `<div class="cap ${value.supported ? 'ok' : 'fail'}">
              ${key}: ${value.supported ? '✅' : '❌'}
            </div>`
          ).join('');
        }
      });
    });
  }
}

customElements.define('ocpp-inspector', OcppInspector);
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
│   ├── chargemesh-ir/                         # EV-IR model
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── network.rs                     # ChargingNetwork
│   │       ├── station.rs                     # ChargingStation
│   │       ├── evse.rs                        # EVSE
│   │       ├── connector.rs                   # Connector
│   │       ├── vehicle.rs                     # Vehicle
│   │       ├── session.rs                     # ChargingSession
│   │       ├── transaction.rs                 # Transaction
│   │       ├── meter.rs                       # Meter, MeterValue
│   │       ├── tariff.rs                      # Tariff
│   │       ├── authorization.rs               # Authorization
│   │       ├── reservation.rs                 # Reservation
│   │       ├── profile.rs                     # ChargingProfile
│   │       ├── capability.rs                  # Capabilities
│   │       ├── error.rs                       # ChargingError (ChargeX)
│   │       ├── firmware.rs                    # Firmware
│   │       ├── energy.rs                      # EnergyConstraint
│   │       └── state_machine/                 # State machines
│   │           ├── mod.rs
│   │           ├── session.rs                 # SessionStateMachine
│   │           ├── station.rs                 # StationStateMachine
│   │           └── connector.rs               # ConnectorStateMachine
│   │
│   ├── chargemesh-ocpp/                       # OCPP Implementation
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── common/
│   │       │   ├── mod.rs
│   │       │   ├── messages.rs
│   │       │   ├── types.rs
│   │       │   ├── errors.rs
│   │       │   └── websocket.rs
│   │       ├── v16/                           # OCPP 1.6
│   │       │   ├── mod.rs
│   │       │   ├── messages.rs                # All 13 messages
│   │       │   ├── types.rs
│   │       │   ├── parser.rs
│   │       │   ├── client.rs
│   │       │   ├── server.rs
│   │       │   └── state_machine.rs
│   │       ├── v201/                          # OCPP 2.0.1
│   │       │   ├── mod.rs
│   │       │   └── types.rs
│   │       └── v21/                           # OCPP 2.1
│   │           ├── mod.rs
│   │           └── types.rs
│   │
│   ├── chargemesh-capability/                 # ⭐ Capability Engine
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── engine/
│   │       │   ├── mod.rs
│   │       │   ├── capability.rs              # CapabilityDefinition, CapabilityState
│   │       │   ├── resolver.rs                # Multi-factor resolution
│   │       │   ├── registry.rs                # Capability registry
│   │       │   └── evaluator.rs               # Rule-based evaluation
│   │       ├── detectors/
│   │       │   ├── mod.rs
│   │       │   ├── protocol_detector.rs       # Protocol-based detection
│   │       │   ├── vendor_detector.rs         # Vendor-based detection
│   │       │   ├── firmware_detector.rs       # Firmware-based detection
│   │       │   └── runtime_detector.rs        # Runtime-based detection
│   │       ├── profiles/
│   │       │   ├── mod.rs
│   │       │   └── model_profiles.rs          # ABB, Siemens profiles
│   │       └── rules/
│   │           ├── mod.rs
│   │           ├── engine.rs                  # Rule engine
│   │           ├── conditions.rs              # Rule conditions
│   │           └── actions.rs                 # Rule actions
│   │
│   └── chargemesh-wasm/                       # WASM module for web
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs                         # Exports for Emerge
│
├── apps/
│   └── chargemesh-cli/                        # Command-line interface
│       ├── Cargo.toml
│       └── src/
│           └── main.rs                        # Parse, Capture, Capability, Version
│
├── web/
│   └── inspector/                             # Web Inspector (Emerge Core)
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
│
├── tests/
│   ├── unit/
│   │   └── core_tests.rs
│   ├── integration/
│   │   ├── ir_tests.rs
│   │   ├── ocpp_tests.rs
│   │   └── capability_tests.rs                # ⭐ Capability tests
│   └── e2e/
│       ├── ocpp_e2e_tests.rs
│       └── capability_e2e_tests.rs            # ⭐ Capability E2E tests
│
└── examples/
    ├── basic_connect.rs
    ├── diagnose_trace.rs
    └── capability_analysis.rs                  # ⭐ Capability example
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

# Show version
chargemesh version
```

---

## Design Principles

### Protocol Independence

Applications should depend on the ChargeMesh model rather than individual charging protocols.

### Capability-First Design

The system should describe what a device can actually do, rather than assuming capabilities from its protocol version. This is the core of Phase 3.

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
| **P3** | Capability Engine | ✅ Complete |
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
- Additional vendor profiles (Alfen, Tritium, etc.)
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

### Phase 3 (Complete)
- ✅ `chargemesh-capability` — Capability Engine
- ✅ Multi-factor detection (protocol, vendor, firmware, runtime)
- ✅ CapabilityRegistry with defaults
- ✅ Rule engine with conditions and actions
- ✅ Vendor profiles (ABB, Siemens)
- ✅ CLI command `capability`
- ✅ Unit, integration, and E2E tests

### Next Steps
- Phase 4: Simulator
- Phase 5: Diagnostics Engine
- Phase 6: Observability Platform
- Phase 7: Web Inspector with Emerge Core + WASM
