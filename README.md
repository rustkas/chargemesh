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

## Why ChargeMesh?

EV charging infrastructure is built from many independent systems:

* charging stations from different vendors;
* different OCPP versions;
* ISO 15118 implementations;
* roaming networks;
* energy management systems;
* payment and authorization systems;
* vendor-specific APIs;
* different firmware versions and capabilities.

Standards improve interoperability, but they do not eliminate implementation differences, protocol errors, vendor extensions, incompatible capabilities, and complex distributed-system failures.

ChargeMesh is designed to provide a common software layer that hides this complexity.

## Core idea

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

* charging stations;
* EVSEs;
* connectors;
* vehicles;
* charging sessions;
* transactions;
* meter values;
* tariffs;
* authorization;
* reservations;
* charging profiles;
* capabilities;
* errors;
* energy constraints.

## Architecture

ChargeMesh is organized into several layers:

```text
┌───────────────────────────────────────────┐
│              Applications                 │
├───────────────────────────────────────────┤
│         Unified ChargeMesh API            │
├───────────────────────────────────────────┤
│         Diagnostics / Observability       │
├───────────────────────────────────────────┤
│       Capability & State Engine            │
├───────────────────────────────────────────┤
│          Universal EV-IR Model             │
├───────────────────────────────────────────┤
│      Protocol Adapter / Gateway Layer      │
├────────────┬────────────┬─────────────────┤
│   OCPP     │ ISO 15118  │      OCPI       │
└────────────┴────────────┴─────────────────┘
```

## Current goals

The initial development focuses on:

* OCPP 1.6 support;
* a canonical EV charging model;
* charging session state machines;
* protocol message processing;
* capability discovery;
* protocol logging;
* deterministic simulation;
* charging-session diagnostics;
* developer tooling.

## Example

A future ChargeMesh application should be able to work with a charging station without directly depending on its protocol implementation:

```rust
let station = chargemesh.connect(config).await?;

println!("{:?}", station.capabilities());

let session = station.start_charging().await?;

println!("Energy: {}", session.energy());

station.set_power_limit(11_000).await?;
```

The application should not need to know whether the underlying device uses OCPP 1.6, OCPP 2.0.1, OCPP 2.1, or a vendor-specific implementation.

## Repository structure

```text
chargemesh/
├── crates/
│   ├── chargemesh-core
│   ├── chargemesh-ir
│   ├── chargemesh-ocpp
│   ├── chargemesh-ocpi
│   ├── chargemesh-iso15118
│   ├── chargemesh-simulator
│   └── chargemesh-diagnostics
│
├── apps/
│   ├── chargemesh-cli
│   └── chargemesh-inspector
│
├── examples/
├── tests/
└── docs/
```

## Project status

> **Early development**

ChargeMesh is currently a research and development project.

The APIs, EV-IR model, protocol adapters, and architecture are expected to evolve significantly.

Do not use the current versions for production charging infrastructure.

## Development

Requirements:

* Rust stable toolchain;
* Cargo;
* Git.

Clone the repository:

```bash
git clone https://github.com/chargemesh/chargemesh.git
cd chargemesh
```

Build:

```bash
cargo build --workspace
```

Run tests:

```bash
cargo test --workspace
```

## Design principles

### Protocol independence

Applications should depend on the ChargeMesh model rather than individual charging protocols.

### Capability-first design

The system should describe what a device can actually do, rather than assuming capabilities from its protocol version.

### Explicit state machines

Charging infrastructure is a distributed state-machine problem. State transitions should be explicit, observable, and testable.

### Diagnostics as a first-class capability

Protocol traces, state transitions, errors, and device events should be correlated into a single charging-session timeline.

### Edge-first architecture

Core protocol processing should be capable of running close to the charging infrastructure.

### Open source core

The fundamental interoperability layer should remain open source.

## Long-term vision

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
                 │    EV-IR      │
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

The long-term objective is to make EV charging infrastructure programmable through a common runtime rather than through dozens of protocol- and vendor-specific integrations.

## License

TBD.

## Contributing

Contributions, protocol implementations, interoperability reports, test cases, and real-world charging-session traces are welcome.

Please open an issue before implementing a major architectural change.

## Roadmap

* [ ] EV-IR specification
* [ ] OCPP 1.6 core
* [ ] Charging session state machine
* [ ] Capability model
* [ ] OCPP simulator
* [ ] Protocol trace format
* [ ] Diagnostics engine
* [ ] OCPP 2.0.1
* [ ] OCPI
* [ ] ISO 15118
* [ ] OCPP 2.1
* [ ] Web-based Inspector
* [ ] Cloud observability
* [ ] Interoperability test platform
* [ ] Energy-management integrations
* [ ] V2G / V2X support
