# ChargeMesh Architecture

## 1. Overview

ChargeMesh is an open-source infrastructure platform designed to provide universal interoperability across the fragmented EV charging ecosystem. It acts as a middleware layer that abstracts the complexity of multiple protocols (OCPP 1.6, 2.0.1, 2.1, ISO 15118, OCPI) and vendor-specific implementations into a unified programming model.

## 2. Architectural Principles

- **Protocol Agnosticism:** ChargeMesh should not favor any protocol. All protocols are first-class citizens.
- **Loose Coupling:** Protocol adapters are pluggable and can be developed, updated, or replaced independently.
- **Stateful Intelligence:** The system maintains session state and context across protocol boundaries.
- **Observability by Design:** Every interaction is instrumented for diagnostics, monitoring, and debugging.
- **Developer-First API:** The external API should be intuitive and hide underlying complexity.

## 3. High-Level Architecture

```
┌────────────────────────────────────────────────────────────────────┐
│                        APPLICATION LAYER                           │
│  (CSMS, Energy Management, SaaS, Monitoring, Analytics)            │
└─────────────────────────────┬──────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────────────┐
│                    UNIFIED PUBLIC API (REST/gRPC)                  │
│                                                                    │
│  connect() │ discoverCapabilities() │ startCharging() │ stop()     │
│  getSession() │ setPowerLimit() │ getDiagnostics()                 │
└─────────────────────────────┬──────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────────────┐
│                        CORE ENGINE (EV-IR)                         │
│                                                                    │
│  ┌─────────────────┐  ┌─────────────────┐  ┌───────────────────┐   │
│  │  Universal EV   │  │   State Machine │  │   Capability      │   │
│  │  Model (EV-IR)  │  │   Orchestrator  │  │   Discovery       │   │
│  └─────────────────┘  └─────────────────┘  └───────────────────┘   │
│                                                                    │
│  ┌─────────────────┐  ┌─────────────────┐  ┌───────────────────┐   │
│  │  Event Pipeline │  │   Transaction   │  │   Error & Fault   │   │
│  │  & Processing   │  │   Manager       │  │   Management      │   │
│  └─────────────────┘  └─────────────────┘  └───────────────────┘   │
└─────────────────────────────┬──────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                     PROTOCOL ADAPTER LAYER                          │
│                                                                     │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌───────────┐   │
│  │ OCPP 1.6     │ │ OCPP 2.0.1   │ │ OCPP 2.1     │ │ ISO 15118 │   │
│  │ Adapter      │ │ Adapter      │ │ Adapter      │ │ Adapter   │   │
│  └──────────────┘ └──────────────┘ └──────────────┘ └───────────┘   │
│  ┌──────────────┐ ┌──────────────┐ ┌────────────────────────────┐   │
│  │ OCPI Adapter │ │ Vendor API   │ │ MQTT/WebSocket Bridge      │   │
│  │              │ │ Adapter      │ │                            │   │
│  └──────────────┘ └──────────────┘ └────────────────────────────┘   │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                       INFRASTRUCTURE LAYER                          │
│                                                                     │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────┐     │
│  │ WebSocket│ │  MQTT    │ │  HTTP/2  │ │  TLS/SSL │ │  File  │     │
│  │ Server   │ │  Client  │ │  Client  │ │  Manager │ │ System │     │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘ └────────┘     │
└─────────────────────────────────────────────────────────────────────┘
```

## 4. Core Components

### 4.1. Protocol Adapters
Each adapter translates between a specific protocol and the internal EV-IR model:
- **Inbound:** Raw protocol messages → EV-IR events/commands
- **Outbound:** EV-IR actions → Protocol-specific messages

### 4.2. Universal EV Model (EV-IR)
The canonical data model representing all charging entities (see `ev-ir.md`).

### 4.3. State Machine Orchestrator
Manages the lifecycle of charging sessions across different protocol state machines. Acts as a "meta-state machine" that reconciles protocol-specific states.

### 4.4. Capability Discovery
Dynamically discovers and normalizes features supported by the underlying station/protocol.

### 4.5. Event Pipeline
Processes and distributes events to subscribers (logging, analytics, monitoring).

### 4.6. Transaction Manager
Handles billing, metering, and session persistence.

### 4.7. Error & Fault Management
Normalizes and routes errors based on the ChargeX taxonomy (see `error-taxonomy.md`).

## 5. Data Flow

1. **Connection Establishment:** Client connects via Unified API → Adapter establishes protocol connection.
2. **Capability Discovery:** Adapter queries station → Normalizes capabilities → Exposes to client.
3. **Charging Request:** Client sends `startCharging()` → State Machine transitions → Adapter sends protocol-specific messages → Station responds.
4. **Event Processing:** Raw events → Adapter normalizes → Core Engine processes → Notifies subscribers.

## 6. Technology Stack

| Layer | Technology |
|-------|------------|
| Frontend | TypeScript, React, WASM |
| API | REST (Axum), gRPC (Tonic), WebSocket |
| Backend | Rust (Tokio, Serde, Tracing) |
| Data | PostgreSQL, Redis, NATS/Kafka |
| Infrastructure | Docker, Kubernetes, Terraform |
| Observability | Prometheus, Grafana, ELK, Jaeger |

## 7. Non-Functional Requirements

- **Scalability:** Horizontal scaling of protocol adapters and core services.
- **Resilience:** Graceful degradation, retry logic, and circuit breakers.
- **Security:** TLS everywhere, token-based authentication, secure key management.
- **Performance:** Sub-second latency for critical operations (start/stop charging).
- **Extensibility:** Plugin system for custom protocol adapters and vendor-specific logic.
- **Observability:** Structured logging, metrics (Prometheus), distributed tracing (Jaeger).

