Create one horizontal animation strip for Codex pet `gamjabot`, state `jumping`.

Use the attached canonical base for identity. Use the attached layout guide only for slot count, spacing, centering, and padding; do not draw the guide.

Output exactly 5 full-body frames in one left-to-right row on flat pure cyan #00FFFF. Treat the row as 5 invisible equal-width slots: one centered complete pose per slot, evenly spaced, with no overlap, clipping, empty slots, labels, or borders.

Identity: same pet in every frame: 원작 감자 캐릭터에 충실한 순수 흑백 픽셀 펫. 매우 큰 비대칭 흰 눈, 작은 검은 점동공, 아래쪽의 굵은 원형 O자 입, 둥근 흰 머리와 가느다란 검은 팔다리. 기본 본체에는 모자, 트로피, 소품, 색상, 텍스트가 전혀 없어야 한다. 향후 별도 액세서리 오버레이를 위해 머리 외곽과 손 주변을 단순하고 비워 둔다.. Preserve silhouette, face, proportions, markings, palette, material, style, and props.
Style: Pet-safe sprite: compact full-body mascot, readable in a 192x208 cell, clear silhouette, simple face, stable palette/materials, and crisp edges for chroma-key extraction. Style `pixel`: Pixel-art-adjacent digital mascot with a chunky silhouette, simple dark outline, limited palette, flat cel shading, and visible stepped edges. User style notes: crisp authentic pixel art, strict black and white only, no grayscale antialiasing, chunky readable pixels, transparent-ready silhouette, preserve the awkward lovable hand-drawn expression.
Animation continuity: keep apparent pet scale and baseline stable within the row unless the state itself intentionally changes vertical position, such as `jumping`. Move the pose within the slot instead of redrawing the pet larger or smaller frame to frame.

State action: Hover jump loop: anticipation, lift, airborne peak, descent, and settle through body height.

State requirements:
- Show the jump through pose and vertical body position only: anticipation, lift, airborne peak, descent, settle.
- Do not draw ground shadows, contact shadows, drop shadows, oval shadows, landing marks, dust, smears, bounce pads, or motion marks under the pet.
- Keep the background outside the pet perfectly flat chroma key with no darker key-colored patches.

Clean extraction: crisp opaque edges, safe padding, no scenery, text, guide marks, checkerboard, shadows, glows, motion blur, speed lines, dust, detached effects, stray pixels, or chroma-key colors inside the pet.
