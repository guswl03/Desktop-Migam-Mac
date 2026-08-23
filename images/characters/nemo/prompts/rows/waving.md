Create one horizontal animation strip for Codex pet `nemo`, state `waving`.

Use the attached canonical base for identity. Use the attached layout guide only for slot count, spacing, centering, and padding; do not draw the guide.

Output exactly 4 full-body frames in one left-to-right row on flat pure cyan #00FFFF. Treat the row as 4 invisible equal-width slots: one centered complete pose per slot, evenly spaced, with no overlap, clipping, empty slots, labels, or borders.

Identity: same pet in every frame: 원작 네모 캐릭터에 충실한 순수 흑백 픽셀 펫. 살짝 비뚤어진 커다란 흰 사각형 머리, 굵고 날카로운 검은 V자 눈썹, 짧은 세로 점눈 두 개, 양 끝이 세로로 올라간 짧은 가로 입, 머리 아래의 작은 흰 사다리꼴 몸통, 짧고 가느다란 검은 팔다리. 무뚝뚝하고 단호하지만 믿음직한 작업 감독관. 기본 본체에는 소품, 액세서리, 색상, 텍스트가 없어야 한다.. Preserve silhouette, face, proportions, markings, palette, material, style, and props.
Style: Pet-safe sprite: compact full-body mascot, readable in a 192x208 cell, clear silhouette, simple face, stable palette/materials, and crisp edges for chroma-key extraction. Style `pixel`: Pixel-art-adjacent digital mascot with a chunky silhouette, simple dark outline, limited palette, flat cel shading, and visible stepped edges. User style notes: crisp authentic 16-bit pixel art, strict black and white only, no grayscale antialiasing, chunky readable pixels, preserve slightly hand-drawn crooked square silhouette and stern expression, transparent-ready.
Animation continuity: keep apparent pet scale and baseline stable within the row unless the state itself intentionally changes vertical position, such as `jumping`. Move the pose within the slot instead of redrawing the pet larger or smaller frame to frame.

State action: Greeting loop: paw or limb down, raised, tilted, and returning in a friendly attention gesture.

State requirements:
- Show the greeting through paw, hand, wing, or limb pose only.
- Do not draw wave marks, motion arcs, lines, sparkles, symbols, or floating effects around the gesture.

Clean extraction: crisp opaque edges, safe padding, no scenery, text, guide marks, checkerboard, shadows, glows, motion blur, speed lines, dust, detached effects, stray pixels, or chroma-key colors inside the pet.
