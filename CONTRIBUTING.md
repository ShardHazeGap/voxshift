# Contributing to VoxShift

Thanks for helping make the best free AI voice changer even better! 💜

## Ways to contribute

| You are… | You can… |
|---|---|
| A user | Report bugs, suggest features, vote in [Discussions](https://github.com/ShardHazeGap/voxshift/discussions) |
| A voice creator | Submit voices to the Voice Hub (see *Voice submissions* below) |
| A translator | Add or improve a locale in `locales/` and translate `README.<lang>.md` |
| A developer | Fix a [`good first issue`](https://github.com/ShardHazeGap/voxshift/labels/good%20first%20issue), improve latency, add a backend |
| A designer | Improve the UI — sources live in [`design/`](design notes) |

## Development setup

```bash
git clone https://github.com/ShardHazeGap/voxshift.git
cd voxshift
# toolchain: Rust stable, Node 20+, pnpm, Python 3.11 (Voice Lab only)
pnpm install
pnpm tauri dev
```

## Pull request checklist

- [ ] One logical change per PR, linked to an issue (`Closes #123`)
- [ ] `pnpm lint` and `cargo test` pass
- [ ] UI changes include an updated screenshot (re-render with `python design/render.py .` — see `design notes`)
- [ ] User-facing changes are added to `CHANGELOG.md` under **Unreleased**
- [ ] Commits follow [Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `docs:` …)

## Voice submissions

Only submit voices that you created yourself or that are licensed for redistribution.
**Voices that imitate real, identifiable people (celebrities, streamers, politicians) are not accepted.**
Include: model file, 10-second preview, licence, category, short description.

## Code of conduct

Be kind and constructive. Harassment, hate speech and promoting misuse of voice cloning are not tolerated.
