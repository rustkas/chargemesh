# Error Taxonomy for ChargeMesh

## 1. Problem Statement
The EV charging industry suffers from fragmented error reporting. A "Connector Failure" might be reported as `ConnectorLockFailure` (OCPP 1.6), a custom vendor code, or a generic OCPP 2.x error. ChargeMesh normalizes these heterogeneous error codes into a unified taxonomy (the **EV-IR Error Model**).

## 2. The ChargeX Minimum Required Error Codes (MRECs)
To streamline diagnostics, ChargeMesh adopts the **ChargeX Consortium's Minimum Required Error Codes (MRECs)** as its base standard for error classification.

All MRECs begin with the prefix `CX` followed by a 3-digit code.

### 2.1. Core MREC Categories (Functional Classification)

#### Hardware Failures (Physical Layer)
- `CX001` **ConnectorLockFailure:** Connector is jammed or not locking properly.
- `CX002` **CableOverheating:** Temperature sensor at the cable/plug exceeded threshold.
- `CX003` **ContactFailure:** Poor contact between connector and vehicle inlet (high resistance).
- `CX004` **HighTemperature:** Thermal throttling or shutdown (Internal environmental issue).
- `CX005` **GroundFailure:** Ground monitoring relay has tripped (RCD/GFCI).
- `CX006` **InternalError:** Generic catch-all for internal software bugs (OCPP 1.6 `InternalError`).
- `CX007` **EVCommunicationError:** Communication lost with the Electric Vehicle (often ISO 15118 related).
- `CX008` **PowerSwitchFailure:** Relay/contactor failed to engage or disengage.
- `CX009` **PowerMeterFailure:** Metering device not responding or providing invalid data.
- `CX010` **OverCurrentFailure:** Current exceeded maximum allowed limit.
- `CX011` **OverVoltage:** Voltage exceeded maximum allowed limit.
- `CX012` **UnderVoltage:** Voltage dropped below minimum allowed limit.
- `CX013` **FrequencyFailure:** Grid frequency outside acceptable range.

#### Communication Errors (Protocol Layer)
- `CX020` **NetworkTimeout:** No response from the server/EV within timeout period.
- `CX021` **ProtocolError:** Invalid message format or unsupported protocol version.
- `CX022` **SecurityError:** TLS handshake failed or certificate validation failed.
- `CX023` **MessageSequenceError:** Message received out of expected order (state machine violation).
- `CX024` **MessageSigningError:** Digital signature verification failed (ISO 15118/OCPP 2.x).

#### Authorization & Billing Errors
- `CX030` **InvalidToken:** RFID/Token not recognized or expired.
- `CX031` **AuthorizationFailed:** Token valid but authorization denied (e.g., insufficient funds).
- `CX032` **CertificateExpired:** ISO 15118 contract certificate expired.
- `CX033` **CertificateRevoked:** ISO 15118 certificate revoked.
- `CX034` **PaymentFailure:** Transaction declined by payment provider.

#### System & Configuration Errors
- `CX040` **ConfigurationMismatch:** Protocol settings don't match the station's capabilities.
- `CX041` **FirmwareIncompatibility:** Firmware version not supported by the CSMS.
- `CX042` **StorageError:** Failed to write to local storage (logs, configurations).
- `CX043` **ClockSynchronizationError:** Station clock not synchronized with NTP.

#### External Constraints & Smart Charging
- `CX050` **GridConstraintViolation:** Requested power exceeds grid capacity.
- `CX051` **LoadSheddingActive:** Charging paused due to load management.
- `CX052` **SolarGenerationInsufficient:** Not enough solar power to deliver requested energy (V2G contexts).
- `CX053` **BatteryFull:** EV battery full, cannot accept more energy.
- `CX054` **BatteryTemperatureTooHigh/Low:** EV battery not in optimal temperature range for charging.

#### Roaming Errors (OCPI)
- `CX060` **RoamingServerUnreachable:** OCPI endpoint not responding.
- `CX061` **RoamingAuthenticationFailed:** OCPI token not valid at this roaming partner.
- `CX062` **TariffMismatch:** Tariff information inconsistent between CPO and eMSP.
- `CX063` **CDRSubmissionFailed:** Could not submit Charging Detail Record to eMSP.

## 3. Protocol-Specific Error Mapping

### 3.1. OCPP 1.6 to ChargeMesh
ChargeMesh translates OCPP 1.6 `ChargePointErrorCode` values to the `CX` standard. If no suitable `CX` code exists, the `vendorErrorCode` field (as recommended by ChargeX) is used to pass the raw data.

*Example Mapping:*
```text
OCPP 1.6 "ConnectorLockFailure"     → ChargeMesh: CX001
OCPP 1.6 "HighTemperature"          → ChargeMesh: CX004
OCPP 1.6 "EVCommunicationError"     → ChargeMesh: CX007
OCPP 1.6 "PowerMeterFailure"        → ChargeMesh: CX009
OCPP 1.6 "OverCurrentFailure"       → ChargeMesh: CX010
OCPP 1.6 "OverVoltage"              → ChargeMesh: CX011
OCPP 1.6 "UnderVoltage"             → ChargeMesh: CX012
OCPP 1.6 "OtherError"               → ChargeMesh: VendorErrorCode (passthrough with CX099)
```

### 3.2. OCPP 2.0.1 / 2.1
OCPP 2.x has a more robust error handling mechanism with structured error messages and `errorCode` definitions. ChargeMesh maps these to the functional classifications of the ChargeX taxonomy for clear interpretation of **who** is responsible for fixing the error (e.g., Manufacturer vs. Operator).

*Example Mapping:*
```text
OCPP 2.0.1 "Rejected" (BootNotification)  → ChargeMesh: CX030 (InvalidToken) or CX040 (ConfigMismatch)
OCPP 2.0.1 "FirmwareUpdateFailed"         → ChargeMesh: CX041 (FirmwareIncompatibility)
OCPP 2.0.1 "SecurityError"                → ChargeMesh: CX022 (SecurityError)
```

## 4. Error Resolution Responsibility (ChargeX Model)
Following the ChargeX model, each error in ChargeMesh will be tagged with a **Responsibility Entity**:
- **OEM (Manufacturer):** Hardware/Physical issues (e.g., `CX001 ConnectorLockFailure`).
- **CPO (Operator):** Network/Config issues (e.g., `CX004 HighTemperature` due to lack of cooling maintenance).
- **eMSP (Provider):** Authorization/Billing issues (e.g., `CX030 InvalidToken`).
- **User/Driver:** EV-specific issues (e.g., `CX053 BatteryFull`).
- **Grid/Utility:** Energy availability issues (e.g., `CX050 GridConstraintViolation`).

## 5. Implementation in ChargeMesh API

```typescript
// ChargeMesh normalizes raw protocol errors into a unified object
interface ChargeMeshError {
  code: string;                 // e.g., 'CX001'
  type: 'Hardware' | 'Communication' | 'Authorization' | 'Configuration' | 'External' | 'Roaming';
  description: string;          // Human-readable
  source: 'EVSE' | 'EV' | 'CSMS' | 'Roaming' | 'Grid';
  severity: 'Critical' | 'Warning' | 'Info';
  timestamp: DateTime;
  raw: any;                     // Original protocol error payload
  recommendedAction?: string;   // Suggested remediation
  resolved: boolean;
}
```

## 6. Error Analytics & Reporting
ChargeMesh aggregates errors to provide:
- **Dashboards:** Top error types, error rates by station/model/protocol.
- **Alerts:** Notify operators when error rates exceed thresholds.
- **Root Cause Analysis:** Correlate errors with time, grid events, firmware versions.
- **Predictive Maintenance:** Identify patterns leading to hardware failures.
