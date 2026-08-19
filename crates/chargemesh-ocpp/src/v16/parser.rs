//! OCPP 1.6 Message Parser

use super::*;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageDirection {
    Incoming,
    Outgoing,
}

#[derive(Debug, Clone)]
pub struct ParsedMessage {
    pub raw: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub direction: MessageDirection,
    pub message: OcppMessage,
}

pub fn parse_ocpp_message(raw: &str) -> Result<ParsedMessage, String> {
    let timestamp = chrono::Utc::now();
    
    // Determine direction
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
                
                Ok(OcppMessage::Call { message_id, action, payload })
            }
            3 => {
                if array.len() < 3 {
                    return Err("Invalid CallResult message".to_string());
                }
                let message_id = array[1].as_str()
                    .ok_or_else(|| "Invalid message ID".to_string())?
                    .to_string();
                let payload = array[2].clone();
                
                Ok(OcppMessage::CallResult { message_id, payload })
            }
            4 => {
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
                
                Ok(OcppMessage::CallError { 
                    message_id, 
                    error_code, 
                    error_description, 
                    details,
                })
            }
            _ => Err(format!("Unknown message type: {}", msg_type)),
        }
    }
}
