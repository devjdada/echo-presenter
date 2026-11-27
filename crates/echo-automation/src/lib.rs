use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AutomationAction {
    NextSlide,
    SetLayerOpacity { layer: usize, opacity: f32 },
    TriggerClip { clip_id: String },
}

pub fn parse_osc_address(addr: &str, arg_f32: Option<f32>) -> Option<AutomationAction> {
    match addr {
        "/slide/next" => Some(AutomationAction::NextSlide),
        "/layer/1/opacity" => Some(AutomationAction::SetLayerOpacity {
            layer: 1,
            opacity: arg_f32.unwrap_or(1.0).clamp(0.0, 1.0),
        }),
        _ if addr.starts_with("/clip/trigger/") => Some(AutomationAction::TriggerClip {
            clip_id: addr.trim_start_matches("/clip/trigger/").to_string(),
        }),
        _ => None,
    }
}

pub fn build_artnet_dmx_packet(universe: u16, channels: &[u8]) -> Vec<u8> {
    let len = channels.len().min(512);
    let mut pkt = Vec::with_capacity(18 + len);
    pkt.extend_from_slice(b"Art-Net\0");
    pkt.extend_from_slice(&0x5000u16.to_le_bytes()); // OpDmx
    pkt.extend_from_slice(&14u16.to_be_bytes()); // Protocol version 14
    pkt.push(0); // Sequence
    pkt.push(0); // Physical
    pkt.extend_from_slice(&universe.to_le_bytes());
    pkt.extend_from_slice(&(len as u16).to_be_bytes());
    pkt.extend_from_slice(&channels[..len]);
    pkt
}

pub fn map_stream_deck_key(key_idx: u8) -> Option<AutomationAction> {
    match key_idx {
        0 => Some(AutomationAction::NextSlide),
        1 => Some(AutomationAction::SetLayerOpacity { layer: 1, opacity: 0.0 }),
        2 => Some(AutomationAction::TriggerClip { clip_id: "clip-1".into() }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osc_mapping() {
        assert_eq!(parse_osc_address("/slide/next", None), Some(AutomationAction::NextSlide));
        assert_eq!(
            parse_osc_address("/layer/1/opacity", Some(0.75)),
            Some(AutomationAction::SetLayerOpacity { layer: 1, opacity: 0.75 })
        );
    }

    #[test]
    fn test_artnet_and_stream_deck() {
        let pkt = build_artnet_dmx_packet(0, &[255, 128, 0]);
        assert_eq!(&pkt[..8], b"Art-Net\0");
        assert_eq!(pkt.len(), 21);
        assert_eq!(map_stream_deck_key(0), Some(AutomationAction::NextSlide));
    }
}
