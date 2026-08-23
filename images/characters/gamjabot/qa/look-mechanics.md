# Gamjabot look mechanics

Gamjabot is a black-and-white pixel humanoid with two oversized asymmetric physical eyeballs, a round head, a circular O-shaped mouth, a narrow torso, and thin limbs. The feet and lower torso remain anchored to one stable baseline. No accessories or props are part of the base identity.

The eyeballs lead each gaze as complete physical forms: pupils, eye rims, and visible eye-white distribution change together. The head follows with a small yaw or pitch, then the neck and upper torso follow by one restrained pixel-art step. The mouth keeps its circular identity and may shift slightly with head orientation. Whole-sprite rotation, skewing, and broad raster deformation are forbidden.

- 000 up: pupils and upper eye surfaces aim upward; eyelids compress slightly at the top; chin lifts while feet remain fixed.
- 090 screen-right: pupils, eye mass, nose/face center, and head turn toward the image's right edge; the left side of the head becomes slightly more visible.
- 180 down: pupils and eye surfaces aim down; upper eyelids lower; chin tucks toward the torso.
- 270 screen-left: pupils, eye mass, nose/face center, and head turn toward the image's left edge; the right side of the head becomes slightly more visible.

Every 22.5-degree step interpolates the neighboring cardinal families evenly. The lower body, scale, baseline, black-white palette, pixel density, and accessory-free silhouette stay stable. The final 337.5 pose must sit one even step before 000 without a snap.
