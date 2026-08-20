# EV-IR: Universal Electric Vehicle Interoperability Runtime Model

## 1. Purpose

The **EV-IR (Electric Vehicle Interoperability Runtime)** is the canonical data model of ChargeMesh. It serves as the single source of truth for all charging-related entities, independent of underlying protocols or vendor-specific implementations.

The EV-IR enables:

- **Protocol Translation:** Mapping any protocol to the same objects.
- **Unified APIs:** Consistent interfaces for developers.
- **Stateful Context:** Maintaining session state across protocol boundaries.
- **Intelligent Orchestration:** Capability discovery, state management, and error handling.

## 2. Core Entity Model

```
┌────────────────────────────────────────────────────────┐
│                    Station                             │
│  + id: string                                          │
│  + vendor: string                                      │
│  + model: string                                       │
│  + firmwareVersion: string                             │
│  + protocol: ProtocolInfo                              │
│  + capabilities: Capability[]                          │
│  + evses: EVSE[]                                       │
│  + status: StationStatus                               │
│  + location: GeoLocation                               │
└───────────────────┬────────────────────────────────────┘
                    │
                    │ 1..*
                    ▼
┌────────────────────────────────────────────────────────┐
│                    EVSE                                │
│  + id: string                                          │
│  + connectors: Connector[]                             │
│  + maxPower: number                                    │
│  + supportedProtocols: ProtocolInfo[]                  │
│  + energyMeter: Meter                                  │
│  + status: EVSEStatus                                  │
│  + capabilities: Capability[]                          │
└───────────────────┬────────────────────────────────────┘
                    │
                    │ 1..*
                    ▼
┌────────────────────────────────────────────────────────┐
│                  Connector                             │
│  + id: string                                          │
│  + type: ConnectorType (e.g., Type2, CCS, CHAdeMO)     │
│  + status: ConnectorStatus                             │
│  + currentSession?: Session                            │
│  + capabilities: Capability[]                          │
│  + power: PowerInfo                                    │
└───────────────────┬────────────────────────────────────┘
                    │
                    │ 0..1
                    ▼
┌────────────────────────────────────────────────────────┐
│                   Session                              │
│  + id: string                                          │
│  + stationId: string                                   │
│  + evseId: string                                      │
│  + connectorId: string                                 │
│  + startTime: DateTime                                 │
│  + endTime?: DateTime                                  │
│  + status: SessionStatus                               │
│  + authorization: AuthorizationInfo                    │
│  + meterStart: Meter                                   │
│  + meterStop?: Meter                                   │
│  + energyConsumed: number                              │
│  + tariff: Tariff                                      │
│  + smartChargingProfile?: Profile                      │
│  + events: SessionEvent[]                              │
└────────────────────────────────────────────────────────┘
```

## 3. Entity Details

### 3.1. Station

- **Description:** Physical charging station (may contain multiple EVSEs).
- **Key Fields:**
  - `id`: Unique identifier (may be OCPP `chargePointId`).
  - `protocol`: Protocol information (version, security profile).
  - `capabilities`: List of supported features.
  - `status`: Overall station status (Online, Offline, Booted, Faulted).
  - `evses`: List of EVSE units.

### 3.2. EVSE (Electric Vehicle Supply Equipment)

- **Description:** Individual charging unit within a station.
- **Key Fields:**
  - `maxPower`: Maximum power (kW) the EVSE can deliver.
  - `connectors`: List of physical connectors (Type2, CCS, etc.).
  - `status`: EVSE-level status (Available, Preparing, Charging, Faulted).

### 3.3. Connector

- **Description:** Physical cable/plug connecting to the vehicle.
- **Key Fields:**
  - `type`: Connector standard (Type2, CCS, CHAdeMO, GB/T).
  - `currentSession`: Active session if charging (null if idle).
  - `power`: Current power measurement (voltage, current, power factor).

### 3.4. Session

- **Description:** A charging transaction from start to stop.
- **Key Fields:**
  - `authorization`: RFID token, Plug & Charge certificate, or OCPI token.
  - `energyConsumed`: Total energy delivered (kWh).
  - `meterStart/meterStop`: Starting and ending meter readings.
  - `tariff`: Pricing information (from OCPI or CSMS).
  - `events`: Timeline of significant events (start, stop, errors, alerts).

### 3.5. Meter

- **Description:** Energy measurement data.
- **Key Fields:**
  - `timestamp`: Time of measurement.
  - `energy`: Total energy (kWh).
  - `power`: Instantaneous power (kW).
  - `voltage`: Voltage (V).
  - `current`: Current (A).
  - `signed`: Whether the data is digitally signed (ISO 15118/OCPP 2.x requirement).

### 3.6. Tariff

- **Description:** Pricing model for charging.
- **Key Fields:**
  - `type`: Flat rate, time-of-day, tiered, etc.
  - `currency`: Currency code (e.g., USD, EUR).
  - `energyPrice`: Price per kWh.
  - `timePrice`: Price per minute/hour.
  - `parkingPrice`: Price for parking (if applicable).
  - `availability`: Time-of-day restrictions (from OCPI).

### 3.7. Capability

- **Description:** Supported features (see `capabilities.md` for full taxonomy).
- **Key Fields:**
  - `name`: Capability identifier (e.g., SMART_CHARGING, PLUG_AND_CHARGE).
  - `supported`: Boolean.
  - `parameters`: Optional parameters (e.g., `maxPower`, `supportedModes`).

### 3.8. PowerInfo

- **Description:** Real-time power delivery data.
- **Key Fields:**
  - `power`: Active power (kW).
  - `reactivePower`: Reactive power (kVar).
  - `voltage`: Voltage (V).
  - `current`: Current (A).
  - `frequency`: Frequency (Hz).

## 4. State Management

The EV-IR includes state machines for:

- **Station Lifecycle:** Offline → Booted → Disabled → Enabled → Faulted.
- **EVSE/Connector:** Available → Preparing → Charging → Stopping → Complete → Faulted.
- **Session:** Initializing → Authorizing → Charging → Stopping → Complete → Faulted.

Each state transition is validated against:

- Protocol-specific state machine (e.g., OCPP 1.6, ISO 15118).
- Capability constraints (e.g., `smartCharging` must be supported for dynamic power changes).
- Security policies (e.g., authorization must be valid before starting).

## 5. Event Model

The EV-IR defines a standardized event structure for all protocol events:

```typescript
interface EV-IREvent {
  id: string;
  timestamp: DateTime;
  type: 'Station' | 'EVSE' | 'Connector' | 'Session';
  action: 'Created' | 'Updated' | 'Deleted' | 'Transition';
  payload: any; // Entity snapshot or delta
  source: string; // Protocol adapter ID
}
```

## 6. Extensibility

The EV-IR is designed to be extensible:

- **Custom Fields:** Vendors can add custom fields via `metadata` key-value store.
- **New Entity Types:** Can be added without breaking existing components.
- **Versioning:** The EV-IR schema is versioned to handle future protocol evolutions.
