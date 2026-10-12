# Unigreen Sound app icon

The **Unigreen three-leaf mark** on a Unigreen dark tile, with the app label underneath.

## Palette

| Colour | Hex | Used for |
|---|---|---|
| Unigreen dark | `#0d1a12` | the full-bleed tile |
| Unigreen mint | `#34d399` | the app label |
| Leaf purple / cyan / green | from the Unigreen logo | the mark (unchanged) |

## Geometry

A 512-unit tile (`viewBox="0 0 512 512"`), rounded square with `rx=112`, no border. The mark is
scaled uniformly (never stretched). The macOS renders pad the tile onto Apple's 824/1024 icon
grid; the Windows and Linux renders crop 22 units off each side. Renders at 48 px and below drop
the label and enlarge the mark so it stays legible.

## Provenance

Unigreen logo (`FINAL_LOGO.eps`, Unigreen Power Co., Ltd.). The Unigreen name and logo are
trademarks of Unigreen Power Co., Ltd. and are not covered by this repository's code licenses.

## Files

Every file in this folder keeps its upstream name and pixel size, so `soundcraft.svg`, the PNGs,
`.ico` and `.icns` drop in without code changes. The label text is converted to outlines, so the
SVGs render the same without the Inter font installed.
