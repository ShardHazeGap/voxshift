# Changelog

All notable changes to **VoxShift — free AI voice changer** are documented here.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Planned
- Stream Deck / Touch Portal plugin
- Real-time singing mode
- Zero-shot voice cloning from 10 seconds of audio

## [1.0.0] — 2026-11-25

First public release. 🎉

### Added
- **Real-time AI voice changer** — RVC v3 engine with CUDA, DirectML, Apple Metal and CPU backends; ~30 ms end-to-end latency on a mid-range GPU.
- **Live latency / GPU / CPU meters** and A/B blend between original and AI voice.
- **Voice Lab** — local voice cloning from ~10 minutes of audio with automatic dataset clean-up, training progress, A/B preview and `.pth` / `.onnx` export.
- **Voice Library** with 250+ free voices, search, categories and one-click install; import of community RVC models.
- **VoxShift Virtual Mic** — signed virtual audio driver (Windows), CoreAudio device (macOS), PipeWire/PulseAudio source (Linux).
- **AI noise & echo suppression** before conversion.
- **Soundboard** with global hotkeys (work in exclusive full-screen), dual output (mic + headphones) and per-key voice switching (hold / toggle).
- **Converter** — batch voice change for MP3, WAV, FLAC, OGG, MP4, MKV, MOV (video stream preserved).
- **Text-to-speech** with any voice and emotion presets.
- **Low-CPU mode**, **offline mode**, WASAPI / ASIO drivers, start with OS, signed auto-updates (Stable / Beta / Off).

[Unreleased]: https://github.com/ShardHazeGap/voxshift/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/ShardHazeGap/voxshift/releases/tag/v1.0.0
