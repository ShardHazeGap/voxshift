"""
Nocturne render pipeline — turns HTML designs into README-ready PNG / GIF assets.

Usage:
  python render.py <repo_dir>              # runs every job in <repo_dir>/design/render.json
  python render.py <repo_dir> <job_name>   # runs only one job

render.json format:
{
  "jobs": [
    {"name": "hero", "type": "png", "src": "design/screens/main.html#screen=dashboard",
     "out": "assets/screenshots/dashboard.png", "w": 1440, "h": 900, "scale": 2},
    {"name": "demo", "type": "gif", "src": "design/anim/demo.html",
     "out": "assets/gifs/demo.gif", "w": 1280, "h": 800, "dur": 5000, "fps": 15, "out_w": 960}
  ]
}

GIF timing contract (deterministic, frame-perfect):
  * every CSS animation / transition on the page is paused and seeked to t (ms)
  * if the page defines  window.__seek(t, dur)  it is called first (for JS-driven state:
    typing text, counters, canvas waveforms, class toggles...)
  * design pages should make the loop seamless: state at t=dur == state at t=0
"""
import json, os, shutil, subprocess, sys, tempfile
from pathlib import Path
from playwright.sync_api import sync_playwright


def to_url(repo: Path, src: str) -> str:
    path, _, frag = src.partition("#")
    p = (repo / path).resolve()
    if not p.exists():
        raise FileNotFoundError(p)
    return p.as_uri() + (("#" + frag) if frag else "")


def prepare(page, url):
    page.goto(url, wait_until="networkidle")
    page.evaluate("document.fonts.ready.then(() => true)")
    page.wait_for_timeout(250)


SEEK_JS = """
async ([t, dur]) => {
  if (window.__seek) { await window.__seek(t, dur); }
  for (const a of document.getAnimations()) { a.pause(); a.currentTime = t; }
  await new Promise(r => requestAnimationFrame(() => requestAnimationFrame(r)));
}
"""


def job_png(browser, repo, j):
    ctx = browser.new_context(viewport={"width": j.get("w", 1440), "height": j.get("h", 900)},
                              device_scale_factor=j.get("scale", 2))
    page = ctx.new_page()
    prepare(page, to_url(repo, j["src"]))
    if "t" in j:  # freeze animations at a nice moment
        page.evaluate(SEEK_JS, [j["t"], j.get("dur", 4000)])
    out = repo / j["out"]
    out.parent.mkdir(parents=True, exist_ok=True)
    page.screenshot(path=str(out), omit_background=j.get("transparent", False))
    ctx.close()
    return out


def job_gif(browser, repo, j):
    w, h = j.get("w", 1280), j.get("h", 800)
    dur, fps = j.get("dur", 4000), j.get("fps", 15)
    out_w = j.get("out_w", 960)
    ctx = browser.new_context(viewport={"width": w, "height": h}, device_scale_factor=j.get("scale", 1))
    page = ctx.new_page()
    prepare(page, to_url(repo, j["src"]))
    n = int(dur / 1000 * fps)
    tmp = Path(tempfile.mkdtemp(prefix="nocturne_"))
    try:
        for i in range(n):
            t = i * 1000 / fps
            page.evaluate(SEEK_JS, [t, dur])
            page.screenshot(path=str(tmp / f"f{i:04d}.png"))
        out = repo / j["out"]
        out.parent.mkdir(parents=True, exist_ok=True)
        colors = j.get("colors", 192)
        vf = (f"scale={out_w}:-1:flags=lanczos,split[s0][s1];"
              f"[s0]palettegen=max_colors={colors}:stats_mode=diff[p];"
              f"[s1][p]paletteuse=dither=bayer:bayer_scale=3:diff_mode=rectangle")
        subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", str(fps),
                        "-i", str(tmp / "f%04d.png"), "-vf", vf, "-loop", "0", str(out)], check=True)
        if j.get("mp4"):
            subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", str(fps),
                            "-i", str(tmp / "f%04d.png"), "-vf", f"scale={out_w}:-2",
                            "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "20",
                            str(out.with_suffix(".mp4"))], check=True)
    finally:
        shutil.rmtree(tmp, ignore_errors=True)
        ctx.close()
    return out


def main():
    repo = Path(sys.argv[1]).resolve()
    only = sys.argv[2] if len(sys.argv) > 2 else None
    jobs = json.loads((repo / "design" / "render.json").read_text(encoding="utf-8"))["jobs"]
    with sync_playwright() as p:
        browser = p.chromium.launch()
        for j in jobs:
            if only and j["name"] != only:
                continue
            fn = job_gif if j["type"] == "gif" else job_png
            out = fn(browser, repo, j)
            print(f"[ok] {j['name']:<22} -> {out.relative_to(repo) if out.is_relative_to(repo) else out}  ({os.path.getsize(out)/1024:.0f} KB)")
        browser.close()


if __name__ == "__main__":
    main()
