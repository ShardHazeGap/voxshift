# VoxShift — UI design spec

Source of truth for the VoxShift desktop UI and every image in this repo.
All README screenshots, GIFs, the banner and the social preview are **rendered from the HTML in this folder**: edit the HTML, re-render, commit.

```
design/
├── vx.css                 # VoxShift UI system: tokens + shared components
├── app.html               # full app UI, every screen; switch with #screen=<id>
├── anim/morph.html        # "you → model → voice" marketing loop (GIF)
├── marketing/
│   ├── gen_banner.py      # writes assets/banner.svg + marketing/banner.html
│   ├── banner.html        # generated; renders assets/banner.png with webfonts
│   └── social.html        # 1280×640 social preview / OG image
├── render.json            # render jobs (PNG + GIF)
└── render.py              # Playwright + ffmpeg renderer
```

Render everything (needs `pip install playwright pillow`, `playwright install chromium`, `ffmpeg` on PATH), from the repo root:

```bash
python design/marketing/gen_banner.py   # only after editing the banner
python design/render.py .               # all jobs
python design/render.py . gif-hero      # one job
```

Open `design/app.html#screen=live` in a browser to see the animated UI.

## 1. Direction: studio hardware

VoxShift should look like a piece of audio gear, not a gaming overlay. A big readable "display" with the current voice, waveform lanes, keys, knobs and LED meters. Calm, dense, precise.

| Do | Don't |
|---|---|
| Flat surfaces separated by 1 px hairlines | Gradients of any kind (backgrounds, buttons, text, charts) |
| One accent (orange), used for *state*: live, active slot, playing, primary action | Glows, colored shadows, blurred blobs, aurora backdrops |
| Mono numerals for every number and unit | Colored avatars per voice; voices are identified by name and line icon |
| Neutral greys for everything else | Emoji in UI, rainbow category colors |

## 2. Color

| Token | Value | Use |
|---|---|---|
| `--bg` | `#0b0b0c` | window background |
| `--panel` / `--panel-2` | `#111113` / `#161618` | inputs, selected rows, overlays |
| `--key` / `--key-2` | `#18181b` / `#1d1d21` | slots, pads, buttons (`-2` = active) |
| `--line` / `--line-2` / `--line-3` | `#222225` / `#2c2c31` / `#3a3a40` | hairlines, control borders, inactive bars |
| `--text` / `-2` / `-3` | `#ececee` / `#8c8c94` / `#56565e` | primary / secondary / labels |
| `--acc` | `#ff5b1f` | live state, active slot border, playhead, primary button, knob arc |
| `--acc-ink` | `#140600` | text and icons on orange |
| `--ok` | `#46c77a` | "engine ready" dot and "installed" tag only |

Waveforms: input `#3a3a40`, output `#d9d9de`, the newest 6–8 output bars in `--acc`.

## 3. Type

**Inter Tight** for UI, **JetBrains Mono** for numbers, units, hotkeys, file names, labels.

| Style | Spec |
|---|---|
| Voice name (Live display) | Inter Tight 76 / 600 / −0.045em |
| Page title | Inter Tight 30 / 600 / −0.035em |
| Big readout (training %, latency) | JetBrains Mono 60–84 / 500 / −0.05em |
| Body | Inter Tight 13 / 400–500 |
| Label | JetBrains Mono 10.5 / 500 / +0.08em / uppercase / `--text-3` |
| Values | JetBrains Mono 11–13 / 500 |

## 4. Layout

- Window 1280 × 800 (renders at 2×). Title bar 38 px with the audio route `Mic → VoxShift Virtual Mic → apps` in mono; text tab nav 44 px (active tab = 1 px white underline); status bar 30 px in mono.
- No sidebar. Each screen is a grid of columns divided by hairlines; the right column (330–360 px) is an inspector of stacked sections (`.sec`, 18/22 px padding, label + content).
- Radii: 8 (keys, pads, buttons, fields), 6 (segmented, inputs), 4 (tags, kbd). No radius on panels; they are separated by lines.

## 5. Screens

| `#screen=` | Purpose | Key elements |
|---|---|---|
| `live` | Real-time conversion | LIVE indicator, slot label, Original / AI voice / Blend segmented control, 76 px voice name + mono meta, IN/OUT waveform lanes with orange playhead, 8 quick slots (`Alt+1…8`) with line icons; inspector: Pitch / Formant / Reverb knobs, processing switches, LED meters, listening apps, orange "Voice is on" key |
| `library` | Browse and install voices | Category / engine / source filter list with counts, command search (`Ctrl K`), sort segment, voice table (rating, uses, installed / Get), preview inspector with waveform, sample line, spec list, Install |
| `voicelab` | Local voice cloning | 4-cell step strip (current = orange top rule), mono 84 px progress, epoch / loss / similarity / ETA / GPU / VRAM grid, thin progress bar, similarity (orange) + loss (dashed grey) chart, dataset list, A/B lanes, export, training log, "model ready" toast |
| `soundboard` | Sounds and voice-on-key | 4×3 pads (number, hotkey, mini waveform, name, duration or "Voice" tag); playing pad = orange border + played bars turn white + 2 px orange progress; keycast overlay; output switches, levels, voice-on-key bindings, now playing |
| `converter` | Files and TTS | Dashed drop zone, queue table (type tag, file, voice, progress, speed), export tags + Convert all; TTS text area with orange caret, emotion segment, voice picker, speed / pitch, generated waveform, Generate WAV |
| `settings` | Engine and privacy | Settings list, rows with segmented / select / switch controls, latency budget (27 ms split into capture, noise AI, inference, buffer, output as a segmented bar), benchmark, privacy |

## 6. Components (`vx.css` + `app.html`)

`win`, `tb` (title bar), `nav`, `status`, `ph` (page header), `sec` / `sec-h`, `lbl`, `kbd`, `seg`, `btn` (`.pri` = orange), `sw` (switch), `opt` (label + value + switch row), `sel`, `field` (with `caret`), `tag`, `bar` (3 px progress), `leds` / `meter`, `knob` (SVG, built in JS), `tbl` (selected row = panel-2 + 2 px orange inset), `sq` (monochrome two-letter app/voice chip), `lane` + `playhead`, `ov` (flat overlay: toast, keycast).

States: default · hover (+1 border step) · active/selected (`--key-2` background + orange 1 px border, number turns orange) · disabled (`--text-3`) · playing (orange progress line).

## 7. Motion

| Element | Motion |
|---|---|
| Waveform lanes | Scroll right to left; amplitude follows a speech-like curve that is periodic, so loops are seamless. Each voice shapes the output differently (Titan dense, Unit-7 quantized, Echo quiet) |
| Voice switch | Name, meta, knobs and active slot change on the cut, no transition |
| LED meters | Follow the newest bars of each lane |
| Soundboard hit | Pad scales 1 → 0.97 → 1, played bars turn white left to right, keycast fades in / out |
| Voice Lab | Progress and curve ease in-out; at 72 % of the loop step 4 becomes current, Test model turns orange, toast slides up |

All motion is deterministic via `window.__seek(t, dur)`, which is how `render.py` produces frame-perfect GIF loops.

## 8. Assets

| Asset | Source | Size |
|---|---|---|
| `assets/logo.svg` | hand-written, flat orange key with five bars forming a V | 256×256 |
| `assets/banner.svg` | `marketing/gen_banner.py` (CSS-animated bars, system font stack) | 1280×400 |
| `assets/banner.png` | `marketing/banner.html` | 2560×800 |
| `assets/social-preview.png` | `marketing/social.html` | 1280×640 |
| `assets/screenshots/*.png` | `app.html#screen=…` | 2560×1600 (2×) |
| `assets/gifs/*.gif` | `app.html`, `anim/morph.html` | 960 px wide, 12–15 fps, 6 s loops |
