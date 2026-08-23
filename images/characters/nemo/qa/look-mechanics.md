# Nemo look mechanics

Nemo is a strict black-and-white pixel humanoid with a large slightly crooked square head, a thick V-shaped brow, two short vertical eyes, a short capped horizontal mouth, a small trapezoid torso, and thin limbs. The feet and small torso remain anchored to one stable baseline. No accessories or props belong to the base identity.

The vertical eyes lead attention by shifting together within the square face. The V brow subtly reshapes to reinforce pitch and yaw while remaining recognizable. The square head follows with restrained yaw or pitch, and the small torso follows by a minimal step. Never rotate, skew, or broadly warp the whole sprite or turn the square into a diamond.

- 000 up: both vertical eyes shift upward, brow apex lifts slightly, square head pitches up while its top edge stays mostly horizontal.
- 090 screen-right: both eyes and facial center shift toward the image's right edge; head yaws right and exposes slightly more of its left side.
- 180 down: both eyes shift down, brow lowers, head tucks toward the small torso.
- 270 screen-left: both eyes and facial center shift toward the image's left edge; head yaws left and exposes slightly more of its right side.

Every 22.5-degree step interpolates evenly. Square silhouette, face spacing, scale, pixel density, baseline, black-white palette, and accessory-free identity remain stable. The 337.5 pose must be one smooth step before 000.
