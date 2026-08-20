# Capabilities Model for ChargeMesh

## 1. Overview
This document defines the `Capability` model within the **EV-IR (Electric Vehicle Interoperability Runtime)**. In the ChargeMesh ecosystem, a capability is a discrete function or feature that a Charging Station, EVSE, Connector, or the broader Energy Management System can expose or perform.

The goal of this model is to provide a unified, protocol-agnostic way to discover, query, and control features, regardless of whether they originate from OCPP 1.6, OCPP 2.0.1, OCPP 2.1, ISO 15118, or vendor-specific APIs.

## 2. Core Capability Categories

### 2.1. Electrical & Energy Management
- **Smart Charging:** The ability to modulate power output based on external signals (e.g., grid constraints, local generation).
  - *OCPP 2.1 mapping:* `SmartChargingAvailable`, `SetChargingProfile` with `ChargingStationExternalConstraints`.
  - *OCPP 1.6 mapping:* Basic Charging Profiles (limited).
- **Bidirectional Power Flow (V2X):** Support for Vehicle-to-Grid (V2G) or Vehicle-to-Home (V2H). Primarily associated with **OCPP 2.1** and **ISO 15118-20**.
- **Load Balancing:** Capability to distribute available power across multiple connectors or stations (Site-level, EVSE-level, or Connector-level control).
- **Metering:** Accuracy and granularity of energy measurement (e.g., `PowerMeterFailure` handling).
- **Dynamic Power Adjustment:** Ability to change power limits during an active session (ISO 15118 `ChargeParameterDiscovery`).

### 2.2. Protocol & Interoperability
- **Protocol Support:** Which communication standards are supported:
  - OCPP (1.6, 2.0.1, 2.1)
  - ISO 15118 (Plug & Charge, V2G)
  - OCPI (Roaming)
  - Vendor REST/WebSocket APIs
- **Device Management:**
  - Remote Firmware Update (supported natively in OCPP 2.0.1 vs. less robust in 1.6).
  - Remote Reset and Diagnostics.
- **Diagnostic Capabilities:**
  - Log file retrieval (OCPP `GetDiagnostics`).
  - Remote reboot (OCPP `Reset`).
  - Self-test execution (OCPP 2.0.1+ `TriggerMessage`).

### 2.3. Security & Authorization
- **Authentication Methods:**
  - RFID / Local List (OCPP 1.6 limited support).
  - Plug & Charge (ISO 15118).
  - Remote Authorization via CSMS.
  - OCPI token-based authorization.
- **Security Profiles:** Support for TLS, Basic Authentication, or specific security profiles mandated by OCPP 2.0.1/2.1.
- **Certificate Management:** Ability to manage ISO 15118 contracts and OCPP security certificates (OCPP 2.0.1 `CertificateManagement`).

### 2.4. User Interaction
- **Display & Messaging:** Ability to send text or visual information to the user interface of the charging station (enhanced in OCPP 2.0.1+).
- **Reservation:** Capability to reserve a specific connector for a user/time (OCPP 2.x).
- **Smart Card Management:** Adding/removing RFID cards (OCPP 1.6 `SendLocalList`, OCPP 2.x enhanced).

### 2.5. Smart Charging & Energy Optimization
- **Charging Profiles:** Ability to apply profiles with schedules (OCPP 1.6, 2.x).
- **External Constraints:** Adjust to grid limits, solar generation, or load shedding (OCPP 2.1 specific).
- **Session Scheduling:** Ability to delay charging to off-peak hours.
- **V2G Integration:** Support for discharging to the grid (ISO 15118-20).

### 2.6. Fault & Error Handling
- **Self-Diagnostics:** Built-in error detection and reporting.
- **Error Recovery:** Ability to automatically recover from transient errors.
- **Fault Logging:** Detailed error reporting with timestamps.

## 3. Capability Discovery Flow
ChargeMesh abstracts the discovery process. A developer will use a unified call to check capabilities, regardless of the underlying protocol.

```typescript
// Example: ChargeMesh abstraction
const station = await chargemesh.connect("station-123");
const capabilities = await station.discoverCapabilities();

// Returns a normalized Capability object
console.log(capabilities.smartCharging.supported); // true
console.log(capabilities.smartCharging.supportsV2G); // true (ISO 15118-20)
console.log(capabilities.protocols.supported); // ['ocpp-2.1', 'iso-15118']
console.log(capabilities.security.profiles); // ['tls', 'basic-auth']
```

## 4. Capability Matrix for Supported Protocols

| Capability | OCPP 1.6 | OCPP 2.0.1 | OCPP 2.1 | ISO 15118 | OCPI |
|------------|----------|------------|----------|-----------|------|
| Smart Charging | Limited | Advanced | Advanced | Yes | No |
| V2G/V2H | No | No | Yes | Yes | No |
| Load Balancing | Yes | Yes | Yes | No | No |
| Remote Firmware Update | Yes | Enhanced | Enhanced | No | No |
| Plug & Charge | No | Yes | Yes | Yes | No |
| Reservation | No | Yes | Yes | No | No |
| Display Messaging | Limited | Yes | Yes | No | No |
| Bilateral Metering | No | Yes | Yes | Yes | Yes |
| Certificate Management | No | Yes | Yes | Yes | No |
| OCPI Roaming | No | No | No | No | Yes |
| Tariff Management | No | No | No | No | Yes |

## 5. Capability Persistence
Capabilities are dynamically discovered and cached:
- **Discovery:** During station boot/authentication.
- **Refresh:** Periodically (configurable) or triggered by firmware updates/configuration changes.
- **Storage:** Redis cache (fast lookup) + PostgreSQL (historical tracking).
