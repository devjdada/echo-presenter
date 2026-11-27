use cpal::traits::{DeviceTrait, HostTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioInputDevice {
    pub name: String,
    pub channels: u16,
    pub sample_rate: u32,
}

pub fn list_input_devices() -> Vec<AudioInputDevice> {
    let host = cpal::default_host();
    let Ok(devices) = host.input_devices() else {
        return Vec::new();
    };
    devices
        .filter_map(|d| {
            let name = d.name().ok()?;
            let cfg = d.default_input_config().ok()?;
            Some(AudioInputDevice {
                name,
                channels: cfg.channels(),
                sample_rate: cfg.sample_rate().0,
            })
        })
        .collect()
}

pub fn compute_rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let sum_sq: f32 = samples.iter().map(|s| s * s).sum();
    (sum_sq / samples.len() as f32).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_rms() {
        assert_eq!(compute_rms(&[]), 0.0);
        let rms = compute_rms(&[0.5, -0.5, 0.5, -0.5]);
        assert!((rms - 0.5).abs() < 1e-5);
    }
}
