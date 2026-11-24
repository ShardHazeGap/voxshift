//! ffprobe/ffmpeg invocation layer: analyze media, run two-pass encode.
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, serde::Serialize)]
pub struct MediaInfo {
    pub duration_secs: f64,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub has_audio: bool,
}

pub fn ffprobe(file: &Path) -> Result<MediaInfo, String> {
    let out = Command::new("ffprobe")
        .args(["-v", "quiet", "-print_format", "json", "-show_format", "-show_streams"])
        .arg(file)
        .output().map_err(|e| e.to_string())?;
    let s = String::from_utf8_lossy(&out.stdout).to_string();
    parse_probe(&s).ok_or_else(|| "ffprobe parse error".into())
}

fn parse_probe(raw: &str) -> Option<MediaInfo> {
    let v: serde_json::Value = serde_json::from_str(raw).ok()?;
    let dur = v["format"]["duration"].as_str()?.parse::<f64>().ok()?;
    let streams = v["streams"].as_array()?;
    let vstream = streams.iter().find(|s| s["codec_type"] == "video")?;
    let (w, h) = (vstream["width"].as_u64()? as u32, vstream["height"].as_u64()? as u32);
    let has_audio = streams.iter().any(|s| s["codec_type"] == "audio");
    let fps = vstream["r_frame_rate"].as_str()
        .and_then(|r| r.split('/').next())
        .and_then(|n| n.parse::<f64>().ok()).unwrap_or(30.0);
    Some(MediaInfo { duration_secs: dur, width: w, height: h, fps, has_audio })
}

pub struct EncodeOpts {
    pub bitrate_kbps: u32,
    pub resolution: String,
    pub fps: u32,
    pub out: PathBuf,
}

pub fn run_pass(source: &Path, opts: &EncodeOpts, pass: u8) -> Result<(), String> {
    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-y", "-i"]).arg(source).args(["-c:v", "libx264", "-b:v", &format!("{}k", opts.bitrate_kbps)]);
    if pass == 1 {
        cmd.args(["-pass", "1", "-f", "null"]);
        cmd.arg(if cfg!(windows) { "NUL" } else { "/dev/null" });
    } else {
        cmd.args(["-pass", "2", "-c:a", "aac", "-b:a", "128k"]);
        cmd.arg(&opts.out);
    }
    let st = cmd.status().map_err(|e| e.to_string())?;
    if st.success() { Ok(()) } else { Err(format!("ffmpeg pass {} failed", pass)) }
}
