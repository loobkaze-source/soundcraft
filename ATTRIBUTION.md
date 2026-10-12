# Attribution

Every asset bundled with or committed to SoundCraft is listed here with its author, source and
license. `cargo xtask assets` fails CI when a tracked asset file has no row.

**Rule:** no Avid, Pro Tools, Adobe or other proprietary icons, images, sounds or presets — ever.
Assets must be original, public domain (CC0) or permissively licensed. See `AGENTS.md`.

## Icons

All UI icons (tools, transport, track buttons) are drawn procedurally in
`crates/ui-egui/src/icons.rs` by the SoundCraft contributors (MIT OR Apache-2.0). No UI icon image
files are used. The app icon (`assets/app-icon/`, the Unigreen three-leaf mark on a Unigreen dark tile with the
app label, by Unigreen Power Co., Ltd.) is listed under Files below.

## Audio

The demo session's audio (drums, bass, pad) and its MIDI parts are synthesised in code in
`crates/engine/src/demo.rs` by the SoundCraft contributors (MIT OR Apache-2.0). No sample files
are bundled.

## Fonts

SoundCraft bundles no font files. It uses the default fonts shipped inside the `egui` crate
(Ubuntu-Light: Ubuntu Font Licence 1.0; Hack: MIT; Noto Emoji: OFL-1.1; emoji-icon-font: OFL-1.1/MIT)
and, at runtime, the operating system's own UI fonts when available (never redistributed).

## Files

| Path | Author | Source | License |
|---|---|---|---|
| `assets/brand/unigreen-logo.svg` | Unigreen Power Co., Ltd. | Unigreen logo (FINAL_LOGO.eps) | Unigreen trademark; not covered by MIT OR Apache-2.0 (see NOTICE) |
| `assets/brand/unigreen-logo.png` | Unigreen Power Co., Ltd. | Unigreen logo (FINAL_LOGO.eps) | Unigreen trademark; not covered by MIT OR Apache-2.0 (see NOTICE) |
| `assets/brand/unigreen-logo-white.svg` | Unigreen Power Co., Ltd. | Unigreen logo (FINAL_LOGO.eps) | Unigreen trademark; not covered by MIT OR Apache-2.0 (see NOTICE) |
| `assets/brand/unigreen-logo-white.png` | Unigreen Power Co., Ltd. | Unigreen logo (FINAL_LOGO.eps) | Unigreen trademark; not covered by MIT OR Apache-2.0 (see NOTICE) |
| `assets/brand/unigreen-mark.svg` | Unigreen Power Co., Ltd. | Unigreen logo (FINAL_LOGO.eps) | Unigreen trademark; not covered by MIT OR Apache-2.0 (see NOTICE) |
| `assets/brand/unigreen-mark.png` | Unigreen Power Co., Ltd. | Unigreen logo (FINAL_LOGO.eps) | Unigreen trademark; not covered by MIT OR Apache-2.0 (see NOTICE) |
| `assets/app-icon/icon.svg` | Unigreen Power Co., Ltd. | the Unigreen three-leaf mark on a Unigreen dark tile with the app label | Unigreen trademark; not covered by MIT OR Apache-2.0 (see NOTICE) |
| `packaging/macos/dmg/` (all files) | Unigreen Power Co., Ltd. (layout and generator originally by @XusBadia) | macOS DMG window background with the Unigreen logo and app icon; see `packaging/macos/dmg/README.md` | Unigreen logo: Unigreen trademark (see NOTICE); generator and layout: MIT OR Apache-2.0 |
| `assets/app-icon/soundcraft-1024.png` | Unigreen Power Co., Ltd. | rendered from `icon.svg` | Unigreen trademark; not covered by MIT OR Apache-2.0 (see NOTICE) |
| `assets/app-icon/soundcraft-macos-512.png` | Unigreen Power Co., Ltd. | rendered from `icon.svg` | Unigreen trademark; not covered by MIT OR Apache-2.0 (see NOTICE) |
| `assets/app-icon/soundcraft.icns` | Unigreen Power Co., Ltd. | rendered from `icon.svg` | Unigreen trademark; not covered by MIT OR Apache-2.0 (see NOTICE) |
| `assets/app-icon/soundcraft.ico` | Unigreen Power Co., Ltd. | rendered from `icon.svg` | Unigreen trademark; not covered by MIT OR Apache-2.0 (see NOTICE) |
| `assets/app-icon/hicolor/` | Unigreen Power Co., Ltd. | rendered from `icon.svg` (16–512 px PNGs and the scalable SVG) | Unigreen trademark; not covered by MIT OR Apache-2.0 (see NOTICE) |
| `docs/images/` | SoundCraft contributors | screenshots of SoundCraft rendered from the synthesised demo session | MIT OR Apache-2.0 |
