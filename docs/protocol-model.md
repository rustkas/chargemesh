# Protocol Model for ChargeMesh

## 1. Overview
This document defines the protocol abstraction layer of ChargeMesh. The goal is to provide a consistent, protocol-agnostic interface for all supported communication standards, allowing ChargeMesh to translate between them seamlessly.

## 2. Protocol Ecosystem Overview

### 2.1. OCPP (Open Charge Point Protocol)
- **Versions:** 1.6 (J), 2.0.1, 2.1
- **Transport:** WebSocket
- **Message Format:** JSON (SOAP in older versions)
- **Key Features:**
  - BootNotification, Heartbeat, StatusNotification
  - StartTransaction, StopTransaction
  - RemoteStartTransaction, RemoteStopTransaction
  - Charging Profiles (Smart Charging)
  - Firmware Management
  - Diagnostic (GetDiagnostics, UpdateFirmware)
  - Security Profiles (Basic, TLS, OCPP 2.0.1 security enhancements)

### 2.2. ISO 15118
- **Purpose:** V2G (Vehicle-to-Grid) communication
- **Transport:** TCP/IP, TLS
- **Message Format:** EXI (Efficient XML) encoded, signed/encrypted
- **Key Features:**
  - Plug & Charge (automatic authentication using certificates)
  - Bidirectional power flow (V2G, V2H)
  - Energy transfer scheduling
  - Secure communication
  - Metering and billing
- **Versions:** ISO 15118-2 (current), ISO 15118-20 (enhanced V2G)

### 2.3. OCPI (Open Charge Point Interface)
- **Purpose:** Roaming (interoperability between eMSPs and CPOs)
- **Transport:** REST (HTTP/HTTPS)
- **Message Format:** JSON
- **Key Modules:**
  - Locations: EVSE and connector data
  - Sessions: Transaction information
  - Tariffs: Pricing and tariffs
  - CDRs: Charging Detail Records (billing)
  - Authorizations: Token validation

### 2.4. Vendor-Specific APIs
- **Examples:** ABB, Siemens, Tritium, Alfen
- **Transport:** Various (REST, MQTT, custom WebSocket)
- **Purpose:** Extended diagnostics, proprietary optimizations, or embedded system integration

## 3. Protocol Adapter Architecture

```
┌───────────────────────────────────────────────────┐
│                  PROTOCOL ADAPTER                 │
│                                                   │
│  ┌───────────────────┐     ┌──────────────────┐   │
│  │   Protocol Client │     │   Protocol       │   │
│  │   (WebSocket/HTTP)│     │   State Machine  │   │
│  └────────┬──────────┘     └────────┬─────────┘   │
│           │                         │             │
│           ▼                         ▼             │
│  ┌─────────────────────────────────────────────┐  │
│  │      Message Translator / Normalizer        │  │
│  │  ┌─────────────────┐  ┌──────────────────┐  │  │
│  │  │ Raw → EV-IR     │  │ EV-IR → Raw      │  │  │
│  │  │ (Inbound)       │  │ (Outbound)       │  │  │
│  │  └─────────────────┘  └──────────────────┘  │  │
│  └────────┬────────────────────────────────────┘  │
│           │                                       │
│           ▼                                       │
│  ┌─────────────────────────────────────────────┐  │
│  │      Lifecycle & Capability Mapping         │  │
│  │  - Boot/Handshake Mapping                   │  │
│  │  - State Translation                        │  │
│  │  - Capability Normalization                 │  │
│  └─────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────┘
```

## 4. Protocol Differences & Challenges

| Feature | OCPP 1.6 | OCPP 2.0.1/2.1 | ISO 15118 | OCPI |
|---------|----------|---------------|-----------|------|
| **Authentication** | RFID, PIN, SIM | Enhanced + certificates | Plug & Charge (certificates) | Token-based |
| **Smart Charging** | Basic profiles | Advanced constraints + local generation | Scheduled energy transfer | N/A |
| **Metering** | Raw values | Granular + signed data | Signed meter values | CDRs |
| **Firmware Updates** | Yes (limited) | Enhanced (signed, rollback) | N/A | N/A |
| **Security** | HTTP Basic, TLS | TLS, digital signatures | TLS, PKI, signatures | OAuth2, API keys |
| **Transactions** | Single session | Multi-transaction | Single | Session-based |

## 5. ChargeMesh Abstraction Layers

### 5.1. Inbound Flow (Protocol → EV-IR)
1. Raw message received.
2. Protocol-specific parser validates format.
3. State machine updates local session state.
4. Message translator maps fields to EV-IR objects:
   - `stationId` → Station
   - `connectorId` → Connector
   - `transactionId` → Session
   - `meterStart`/`meterStop` → Meter
   - `chargingProfile` → Tariff/Capability
5. EV-IR event is emitted.

### 5.2. Outbound Flow (EV-IR → Protocol)
1. EV-IR action received (e.g., `startCharging()`).
2. State machine validates transition.
3. Message translator generates protocol-specific message:
   - OCPP: `RemoteStartTransaction`
   - ISO 15118: `ChargeParameterDiscoveryReq`
   - OCPI: `POST /sessions`
4. Protocol client sends message.

### 5.3. Capability Resolution
- Each adapter maintains a capability matrix mapping protocol-specific features to EV-IR capabilities.
- During initialization, the adapter queries the station and populates the matrix.
- The core engine uses this matrix to validate API calls and provide appropriate error messages.
