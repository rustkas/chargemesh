//! Unit tests for chargemesh-core

use chargemesh_core::*;

// ============================================================================
// Types Tests
// ============================================================================

#[test]
fn test_power_creation() {
    let p = Power::new(22000);
    assert_eq!(p.as_watts(), 22000);
    assert_eq!(p.as_kw(), 22.0);
    assert_eq!(format!("{}", p), "22.00 kW");
}

#[test]
fn test_energy_creation() {
    let e = Energy::new(45000);
    assert_eq!(e.as_wh(), 45000);
    assert_eq!(e.as_kwh(), 45.0);
    assert_eq!(format!("{}", e), "45.00 kWh");
}

#[test]
fn test_duration_creation() {
    let d = Duration::new(3665);
    assert_eq!(d.as_seconds(), 3665);
    assert_eq!(format!("{}", d), "1h 1m");
}

#[test]
fn test_temperature_creation() {
    let t = Temperature::new(25.5);
    assert_eq!(t.as_celsius(), 25.5);
    assert_eq!(format!("{}", t), "25.5°C");
}

#[test]
fn test_percentage_creation() {
    let p = Percentage::new(75);
    assert_eq!(p.as_u8(), 75);
    assert_eq!(p.as_ratio(), 0.75);
    assert_eq!(format!("{}", p), "75%");
}

#[test]
fn test_percentage_clamping() {
    let p = Percentage::new(150);
    assert_eq!(p.as_u8(), 100);
}

#[test]
fn test_money_creation() {
    let m = Money::new(12345);
    assert_eq!(m.as_minor_units(), 12345);
    assert_eq!(m.as_major_units(), 123.45);
    assert_eq!(format!("{}", m), "$123.45");
}

// ============================================================================
// Identifiers Tests
// ============================================================================

#[test]
fn test_id_creation() {
    let id = Id::new();
    let uuid = id.as_uuid();
    assert!(!uuid.is_nil());

    let str_id = id.to_string();
    let parsed = Id::parse(&str_id).unwrap();
    assert_eq!(id, parsed);
}

#[test]
fn test_station_id() {
    let id = StationId::new("CP-001");
    assert_eq!(id.as_str(), "CP-001");
    assert!(id.is_valid());

    let id_from_str: StationId = "CP-002".into();
    assert_eq!(id_from_str.as_str(), "CP-002");
}

#[test]
fn test_session_id() {
    let id = SessionId::new();
    assert!(!id.to_string().is_empty());

    let uuid = uuid::Uuid::new_v4();
    let id = SessionId::from_uuid(uuid);
    assert_eq!(id.as_id().as_uuid(), &uuid);
}

// ============================================================================
// Time Tests
// ============================================================================

#[test]
fn test_timestamp_parsing() {
    let ts_str = "2024-01-01T00:00:00Z";
    let ts = parse_timestamp(ts_str).unwrap();
    assert_eq!(format_timestamp(&ts), ts_str);
}

#[test]
fn test_time_range() {
    let start = parse_timestamp("2024-01-01T00:00:00Z").unwrap();
    let end = parse_timestamp("2024-01-01T01:00:00Z").unwrap();
    let range = TimeRange::new(start, end);

    let middle = parse_timestamp("2024-01-01T00:30:00Z").unwrap();
    assert!(range.contains(&middle));

    let before = parse_timestamp("2023-12-31T23:59:59Z").unwrap();
    assert!(!range.contains(&before));
}

// ============================================================================
// Crypto Tests
// ============================================================================

#[test]
fn test_sha256_hash() {
    let hash = Sha256Hash::compute_str("hello");
    let hex = hash.to_hex();
    assert_eq!(hex.len(), 64);

    let parsed = Sha256Hash::from_hex(&hex).unwrap();
    assert_eq!(hash.0, parsed.0);
}

#[test]
fn test_generate_token() {
    let token = generate_token();
    assert_eq!(token.len(), 64); // 32 bytes = 64 hex chars
}