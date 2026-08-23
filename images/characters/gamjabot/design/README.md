# Gamjabot design source

These boards define the visual direction for the production character pack.

- `01-action-board.png`: required and extended pet actions
- `02-emotion-board.png`: expression and application-state vocabulary
- `03-accessory-board.png`: removable accessory concepts
- `04-desktop-pet-ui-kit.png`: speech, timer, warning, tray, and app UI assets

## Production rules

- Keep the body monochrome: pure black and pure white only.
- Each eye contains exactly one intentional solid-black pupil. Do not add grey dots,
  highlights, reflections, texture, scanlines, or scattered pixels.
- Export body frames on a transparent 256x256 canvas with the feet registered at
  `(128, 238)` where the pose is grounded.
- Accessories remain separate transparent 256x256 overlays. Never bake a costume
  into the base body frames.
- Speech bubbles, cards, shadows, and impact effects remain separate UI assets.

Run `scripts/generate-gamjabot-pack.py` with the bundled workspace Python to
rebuild and validate the app-ready pack from the approved production pose sheet.
