import gamjabotAtlas from "../../images/characters/gamjabot/final/spritesheet-extended.webp";
import dragged00 from "../../images/characters/gamjabot/extra/frames/dragged/00.png";
import dragged01 from "../../images/characters/gamjabot/extra/frames/dragged/01.png";
import dragged02 from "../../images/characters/gamjabot/extra/frames/dragged/02.png";
import dragged03 from "../../images/characters/gamjabot/extra/frames/dragged/03.png";
import hardImpact00 from "../../images/characters/gamjabot/extra/frames/hard-impact/00.png";
import focused00 from "../../images/characters/gamjabot/extra/frames/focused/00.png";
import landing00 from "../../images/characters/gamjabot/extra/frames/landing/00.png";
import landing01 from "../../images/characters/gamjabot/extra/frames/landing/01.png";
import landing02 from "../../images/characters/gamjabot/extra/frames/landing/02.png";
import landing03 from "../../images/characters/gamjabot/extra/frames/landing/03.png";
import thrown00 from "../../images/characters/gamjabot/extra/frames/thrown/00.png";
import thrown01 from "../../images/characters/gamjabot/extra/frames/thrown/01.png";
import thrown02 from "../../images/characters/gamjabot/extra/frames/thrown/02.png";
import thrown03 from "../../images/characters/gamjabot/extra/frames/thrown/03.png";
import thrown04 from "../../images/characters/gamjabot/extra/frames/thrown/04.png";
import thrown05 from "../../images/characters/gamjabot/extra/frames/thrown/05.png";

export type PetAnimation =
  | "idle"
  | "running-left"
  | "running-right"
  | "jumping"
  | "dragged"
  | "thrown"
  | "landing"
  | "hard-impact"
  | "focused";

const DISPLAY_CELL_WIDTH = 96;
const DISPLAY_CELL_HEIGHT = 104;
const DISPLAY_ATLAS_WIDTH = 768;
const DISPLAY_ATLAS_HEIGHT = 1144;

type AnimationDefinition =
  | { source: "atlas"; row: number; frames: number }
  | { source: "images"; images: string[]; frames: number };

const animations: Record<PetAnimation, AnimationDefinition> = {
  idle: { source: "atlas", row: 0, frames: 6 },
  "running-right": { source: "atlas", row: 1, frames: 8 },
  "running-left": { source: "atlas", row: 2, frames: 8 },
  jumping: { source: "atlas", row: 4, frames: 5 },
  dragged: { source: "images", images: [dragged00, dragged01, dragged02, dragged03], frames: 4 },
  thrown: { source: "images", images: [thrown00, thrown01, thrown02, thrown03, thrown04, thrown05], frames: 6 },
  landing: { source: "images", images: [landing00, landing01, landing02, landing03], frames: 4 },
  "hard-impact": { source: "images", images: [hardImpact00], frames: 1 },
  focused: { source: "images", images: [focused00], frames: 1 },
};

export interface PetSprite {
  element: HTMLDivElement;
  setAnimation(animation: PetAnimation): void;
  advanceFrame(): void;
}

export function createPetSprite(): PetSprite {
  const element = document.createElement("div");
  element.className = "pet-sprite";
  element.setAttribute("role", "img");
  element.setAttribute("aria-label", "감자봇 데스크톱 펫");
  element.style.backgroundImage = `url("${gamjabotAtlas}")`;
  element.style.backgroundSize = `${DISPLAY_ATLAS_WIDTH}px ${DISPLAY_ATLAS_HEIGHT}px`;

  let animation: PetAnimation = "idle";
  let frame = 0;

  const render = (): void => {
    const definition = animations[animation];
    element.dataset.animation = animation;
    if (definition.source === "atlas") {
      element.style.backgroundImage = `url("${gamjabotAtlas}")`;
      element.style.backgroundSize = `${DISPLAY_ATLAS_WIDTH}px ${DISPLAY_ATLAS_HEIGHT}px`;
      element.style.backgroundPosition = `${-frame * DISPLAY_CELL_WIDTH}px ${-definition.row * DISPLAY_CELL_HEIGHT}px`;
      return;
    }

    element.style.backgroundImage = `url("${definition.images[frame]}")`;
    element.style.backgroundSize = animation === "focused" ? "contain" : `${DISPLAY_CELL_WIDTH}px ${DISPLAY_CELL_HEIGHT}px`;
    element.style.backgroundPosition = animation === "focused" ? "center bottom" : "0 0";
  };

  render();

  return {
    element,
    setAnimation(nextAnimation) {
      if (nextAnimation !== animation) {
        animation = nextAnimation;
        frame = 0;
        render();
      }
    },
    advanceFrame() {
      frame = (frame + 1) % animations[animation].frames;
      render();
    },
  };
}
