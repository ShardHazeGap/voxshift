//! Bitrate and preset engine: maps a target size to encoder settings.
use serde::{Deserialize, Serialize};

pub const FREE_LIMIT: u64 = 20 * 1024 * 1024;
pub const NITRO_BASIC: u64 = 50 * 1024 * 1024;
pub const NITRO_FULL: u64 = 500 * 1024 * 1024;
pub const SAFETY_MARGIN: f64 = 0.03;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Preset {
    DiscordFree,
    NitroBasic,
    NitroFull,
    Custom(u64),
}

impl Preset {
    pub fn target_bytes(self) -> u64 {
        match self {
            Preset::DiscordFree => FREE_LIMIT,
            Preset::NitroBasic => NITRO_BASIC,
            Preset::NitroFull => NITRO_FULL,
            Preset::Custom(b) => b,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Preset::DiscordFree => "Discord Free (20 MB)",
            Preset::NitroBasic => "Nitro Basic (50 MB)",
            Preset::NitroFull => "Nitro (500 MB)",
            Preset::Custom(_) => "Custom",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodePlan {
    pub target_bytes: u64,
    pub duration_secs: f64,
    pub bitrate_kbps: u32,
    pub resolution: String,
    pub fps: u32,
    pub two_pass: bool,
}

pub fn plan(preset: Preset, duration_secs: f64) -> EncodePlan {
    let target = preset.target_bytes();
    let usable = target as f64 * (1.0 - SAFETY_MARGIN);
    let audio_kbps = 128.0;
    let video_bits = (usable * 8.0).max(0.0) - audio_kbps * 1000.0 * duration_secs;
    let bitrate_kbps = (video_bits / 1000.0 / duration_secs.max(0.1)).max(64.0) as u32;
    let (resolution, fps) = pick_profile(bitrate_kbps);
    EncodePlan { target_bytes: target, duration_secs, bitrate_kbps, resolution, fps, two_pass: true }
}

fn pick_profile(kbps: u32) -> (String, u32) {
    match kbps {
        k if k >= 6000 => ("1920x1080".into(), 60),
        k if k >= 3500 => ("1600x900".into(), 60),
        k if k >= 2000 => ("1280x720".into(), 60),
        k if k >= 1000 => ("1280x720".into(), 30),
        k if k >= 600  => ("960x540".into(), 30),
        _ => ("854x480".into(), 30),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn free_limit_roundtrip() {
        let p = plan(Preset::DiscordFree, 25.0);
        assert!(p.bitrate_kbps > 0);
        assert!(p.two_pass);
    }
}
