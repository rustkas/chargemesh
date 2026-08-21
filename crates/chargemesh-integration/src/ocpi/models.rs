//! OCPI data models

use super::*;

// ============================================================================
// Location (CPO → EMSP)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcpiLocation {
    pub id: String,
    pub name: Option<String>,
    pub address: String,
    pub city: String,
    pub country: String,
    pub coordinates: Coordinates,
    pub evses: Vec<OcpiEvse>,
    pub operator: Option<OcpiBusinessDetails>,
    pub owner: Option<OcpiBusinessDetails>,
    pub parking_type: Option<ParkingType>,
    pub access_type: Option<AccessType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcpiEvse {
    pub id: String,
    pub status: OcpiEvseStatus,
    pub capabilities: Vec<OcpiCapability>,
    pub connectors: Vec<OcpiConnector>,
    pub floor_level: Option<String>,
    pub coordinates: Option<Coordinates>,
    pub physical_reference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OcpiEvseStatus {
    Available,
    Blocked,
    Charging,
    Inoperative,
    OutOfOrder,
    Planned,
    Removed,
    Reserved,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OcpiCapability {
    ChargingProfileCapable,
    ReservationCapable,
    RemoteStartStopCapable,
    SmartChargingCapable,
    PlugAndChargeCapable,
    BidirectionalCapable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcpiConnector {
    pub id: String,
    pub connector_type: OcpiConnectorType,
    pub power_type: OcpiPowerType,
    pub max_voltage: f64,
    pub max_amperage: f64,
    pub max_power: f64,
    pub tariff_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OcpiConnectorType {
    CHAdeMO,
    CCS1,
    CCS2,
    GB_T,
    TeslaSupercharger,
    Type1,
    Type2,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OcpiPowerType {
    AC_1_PHASE,
    AC_2_PHASE,
    AC_3_PHASE,
    DC,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParkingType {
    OnStreet,
    ParkingGarage,
    UndergroundGarage,
    ParkingLot,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccessType {
    Free,
    PaymentRequired,
    ResidentsOnly,
    CustomersOnly,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcpiBusinessDetails {
    pub name: String,
    pub website: Option<String>,
    pub logo: Option<OcpiImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcpiImage {
    pub url: String,
    pub thumbnail: Option<String>,
    pub category: OcpiImageCategory,
    pub r#type: OcpiImageType,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OcpiImageCategory {
    Charger,
    Location,
    Operator,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OcpiImageType {
    JPEG,
    PNG,
    GIF,
    SVG,
}

// ============================================================================
// Session (EMSP ↔ CPO)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcpiSession {
    pub id: String,
    pub session_id: String,
    pub station_id: String,
    pub evse_id: String,
    pub connector_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub energy: Option<f64>,
    pub cost: Option<f64>,
    pub currency: Option<String>,
    pub status: OcpiSessionStatus,
    pub auth_method: OcpiAuthMethod,
    pub authorization_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OcpiSessionStatus {
    Pending,
    Active,
    Completed,
    Cancelled,
    Faulted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OcpiAuthMethod {
    AuthRequest,
    WhiteList,
    PlugAndCharge,
}

// ============================================================================
// Tariff (EMSP → CPO)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcpiTariff {
    pub id: String,
    pub currency: String,
    pub tariff_alt_text: Vec<DisplayText>,
    pub price_components: Vec<OcpiPriceComponent>,
    pub energy_price: Option<f64>,
    pub parking_price: Option<f64>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcpiPriceComponent {
    pub r#type: OcpiPriceComponentType,
    pub price: f64,
    pub step_size: f64,
    pub unit: OcpiUnit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OcpiPriceComponentType {
    Time,
    Flat,
    Energy,
    Parking,
    Reservation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OcpiUnit {
    Minutes,
    kWh,
    Hours,
    Days,
    Months,
    Years,
    Percent,
}

// ============================================================================
// CDR (EMSP ↔ CPO)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcpiCdr {
    pub id: String,
    pub session_id: String,
    pub cdr_token: OcpiCdrToken,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub energy: f64,
    pub total_cost: f64,
    pub currency: String,
    pub tariff_id: Option<String>,
    pub cdr_tariff: Option<OcpiTariff>,
    pub charging_periods: Vec<OcpiChargingPeriod>,
    pub status: OcpiCdrStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcpiCdrToken {
    pub uid: String,
    pub r#type: OcpiTokenType,
    pub auth_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OcpiTokenType {
    RFID,
    QRCode,
    App,
    Contract,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OcpiCdrStatus {
    Pending,
    Accepted,
    Rejected,
    Confirmed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcpiChargingPeriod {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub dimensions: Vec<OcpiDimension>,
    pub tariff_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcpiDimension {
    pub r#type: OcpiDimensionType,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OcpiDimensionType {
    Energy,
    Power,
    Time,
    Current,
    Voltage,
}

// ============================================================================
// Common types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayText {
    pub language: String,
    pub text: String,
}