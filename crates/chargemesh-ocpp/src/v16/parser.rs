//! OCPP 1.6 Message Parser

use super::*;
use crate::common::*;
use serde_json::Value;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageDirection {
    Incoming,
    Outgoing,
}

impl fmt::Display for MessageDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageDirection::Incoming => write!(f, "⬅️ "),
            MessageDirection::Outgoing => write!(f, "➡️ "),
        }
    }
}

/// Parsed OCPP message with metadata
#[derive(Debug, Clone)]
pub struct ParsedMessage {
    pub raw: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub direction: MessageDirection,
    pub message: OcppMessage,
}

/// Parse an OCPP 1.6 message from JSON
pub fn parse_ocpp_message(raw: &str) -> Result<ParsedMessage, String> {
    let timestamp = chrono::Utc::now();

    // Determine direction based on message type
    let direction = if raw.contains("CallResult") || raw.contains("CallError") {
        MessageDirection::Incoming
    } else {
        MessageDirection::Outgoing
    };

    let message = OcppMessage::parse(raw)?;

    Ok(ParsedMessage {
        raw: raw.to_string(),
        timestamp,
        direction,
        message,
    })
}

impl OcppMessage {
    /// Parse an OCPP message from JSON string
    pub fn parse(json: &str) -> Result<Self, String> {
        let value: Value = serde_json::from_str(json)
            .map_err(|e| format!("Invalid JSON: {}", e))?;

        let array = value.as_array()
            .ok_or_else(|| "Expected array".to_string())?;

        if array.is_empty() {
            return Err("Empty message".to_string());
        }

        let msg_type = array[0].as_u64()
            .ok_or_else(|| "Invalid message type".to_string())?;

        match msg_type {
            2 => {
                // Call
                if array.len() < 4 {
                    return Err("Invalid Call message".to_string());
                }
                let message_id = array[1].as_str()
                    .ok_or_else(|| "Invalid message ID".to_string())?
                    .to_string();
                let action = array[2].as_str()
                    .ok_or_else(|| "Invalid action".to_string())?
                    .to_string();
                let payload = array[3].clone();

                Ok(OcppMessage::Call(Call { message_id, action, payload }))
            }
            3 => {
                // CallResult
                if array.len() < 3 {
                    return Err("Invalid CallResult message".to_string());
                }
                let message_id = array[1].as_str()
                    .ok_or_else(|| "Invalid message ID".to_string())?
                    .to_string();
                let payload = array[2].clone();

                Ok(OcppMessage::CallResult(CallResult { message_id, payload }))
            }
            4 => {
                // CallError
                if array.len() < 5 {
                    return Err("Invalid CallError message".to_string());
                }
                let message_id = array[1].as_str()
                    .ok_or_else(|| "Invalid message ID".to_string())?
                    .to_string();
                let error_code = array[2].as_str()
                    .ok_or_else(|| "Invalid error code".to_string())?
                    .to_string();
                let error_description = array[3].as_str()
                    .ok_or_else(|| "Invalid error description".to_string())?
                    .to_string();
                let details = array[4].clone();

                Ok(OcppMessage::CallError(CallError {
                    message_id,
                    error_code,
                    error_description,
                    details,
                }))
            }
            _ => Err(format!("Unknown message type: {}", msg_type)),
        }
    }
}

/// Serialize an OCPP message to JSON
pub fn serialize_ocpp_message(message: &OcppMessage) -> Result<String, String> {
    serde_json::to_string(message)
        .map_err(|e| format!("Serialization error: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_boot_notification() {
        let json = r#"[2, "1", "BootNotification", {"chargePointVendor": "ABB", "chargePointModel": "Terra 54"}]"#;
        let parsed = parse_ocpp_message(json).unwrap();

        match parsed.message {
            OcppMessage::Call(call) => {
                assert_eq!(call.message_id, "1");
                assert_eq!(call.action, "BootNotification");
                let req: BootNotificationRequest = serde_json::from_value(call.payload).unwrap();
                assert_eq!(req.charge_point_vendor, "ABB");
                assert_eq!(req.charge_point_model, "Terra 54");
            }
            _ => panic!("Expected Call message"),
        }
    }

    #[test]
    fn test_parse_boot_notification_response() {
        let json = r#"[3, "1", {"status": "Accepted", "currentTime": "2024-01-01T00:00:00Z", "interval": 60}]"#;
        let parsed = parse_ocpp_message(json).unwrap();

        match parsed.message {
            OcppMessage::CallResult(result) => {
                assert_eq!(result.message_id, "1");
                let resp: BootNotificationResponse = serde_json::from_value(result.payload).unwrap();
                assert_eq!(resp.status, RegistrationStatus::Accepted);
                assert_eq!(resp.interval, 60);
            }
            _ => panic!("Expected CallResult message"),
        }
    }

    #[test]
    fn test_parse_error() {
        let json = r#"[4, "1", "InternalError", "Something went wrong", {}]"#;
        let parsed = parse_ocpp_message(json).unwrap();

        match parsed.message {
            OcppMessage::CallError(error) => {
                assert_eq!(error.message_id, "1");
                assert_eq!(error.error_code, "InternalError");
                assert_eq!(error.error_description, "Something went wrong");
            }
            _ => panic!("Expected CallError message"),
        }
    }
}