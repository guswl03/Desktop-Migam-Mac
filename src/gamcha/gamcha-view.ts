import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { costumeById, costumes } from "../costumes/catalog";
import {
  resolveCostumeAlignment,
  type CostumeAlignment,
} from "../costumes/alignment";
import { rarityLabel, rouletteDelay, type GamchaRarity } from "./gamcha-model";

interface GamchaSnapshot {
  tickets: number;
  totalDraws: number;
  ownedCount: number;
  ownedCostumeIds: string[];
  equippedCostumeId: string | null;
  costumeAlignments: Record<string, CostumeAlignment>;
}

interface GamchaDrawResult {
  tickets: number;
  totalDraws: number;
  ownedCount: number;
  costumeId: string;
  rarity: GamchaRarity;
  isNew: boolean;
}

function randomCostume() {
  const random = new Uint32Array(1);
  crypto.getRandomValues(random);
  return costumes[random[0] % costumes.length];
}

function wait(milliseconds: number): Promise<void> {
  return new Promise((resolve) => window.setTimeout(resolve, milliseconds));
}

export async function mountGamcha(container: HTMLElement): Promise<() => void> {
  container.innerHTML = `
    <main class="gamcha-panel">
      <section class="gamcha-bubble" data-rarity="common" aria-labelledby="gamcha-heading">
        <div class="gamcha-speed-lines" aria-hidden="true"></div>
        <div class="gamcha-orbit" aria-hidden="true"></div>
        <div class="gamcha-confetti" aria-hidden="true"></div>
        <button class="gamcha-close" type="button" aria-label="GAMCHA 닫기">×</button>
        <div class="gamcha-stars" aria-hidden="true">✦ ✧ ✦ ✧ ✦ ✧ ✦</div>
        <h1 id="gamcha-heading" class="gamcha-logo" aria-label="GAMCHA!">
          <span>g</span><span>a</span><span>m</span><span>c</span><span>h</span><span>a</span><span>!</span>
        </h1>
        <div class="gamcha-ticket-row"><span>TICKET</span><strong id="gamcha-tickets">0</strong></div>
        <div class="gamcha-stage" aria-live="polite">
          <div class="gamcha-rays" aria-hidden="true"></div>
          <img id="gamcha-costume" alt="추첨한 코스튬" hidden />
          <p id="gamcha-rarity" class="gamcha-rarity">READY</p>
          <p id="gamcha-name" class="gamcha-name">집중 보상을 뽑아보세요</p>
          <p id="gamcha-new" class="gamcha-new"></p>
        </div>
        <button id="gamcha-draw" class="gamcha-draw" type="button">GAMCHA 돌리기!</button>
        <div class="gamcha-wardrobe">
          <label for="gamcha-outfit">OUTFIT</label>
          <select id="gamcha-outfit" aria-label="착용할 코스튬"></select>
          <button id="gamcha-equip" type="button">적용</button>
          <div class="gamcha-alignment" aria-label="코스튬 위치 조정">
            <label>X <output id="gamcha-align-x-value">0</output><input id="gamcha-align-x" type="range" min="-80" max="80" step="1" /></label>
            <label>Y <output id="gamcha-align-y-value">0</output><input id="gamcha-align-y" type="range" min="-80" max="80" step="1" /></label>
            <label>크기 <output id="gamcha-align-size-value">100</output><input id="gamcha-align-size" type="range" min="48" max="180" step="1" /></label>
            <button id="gamcha-align-reset" type="button">위치 초기화</button>
          </div>
          <span id="gamcha-equip-status" role="status"></span>
        </div>
        <div class="gamcha-meta"><span id="gamcha-owned">COLLECTION 0 / ${costumes.length}</span><span>C60 · R25 · E10 · L4 · S1</span></div>
        <p id="gamcha-error" class="gamcha-error" role="alert"></p>
      </section>
    </main>`;

  const bubble = container.querySelector<HTMLElement>(".gamcha-bubble")!;
  const ticketCount = container.querySelector<HTMLElement>("#gamcha-tickets")!;
  const ownedCount = container.querySelector<HTMLElement>("#gamcha-owned")!;
  const image = container.querySelector<HTMLImageElement>("#gamcha-costume")!;
  const rarity = container.querySelector<HTMLElement>("#gamcha-rarity")!;
  const name = container.querySelector<HTMLElement>("#gamcha-name")!;
  const newLabel = container.querySelector<HTMLElement>("#gamcha-new")!;
  const error = container.querySelector<HTMLElement>("#gamcha-error")!;
  const drawButton = container.querySelector<HTMLButtonElement>("#gamcha-draw")!;
  const outfit = container.querySelector<HTMLSelectElement>("#gamcha-outfit")!;
  const equipButton = container.querySelector<HTMLButtonElement>("#gamcha-equip")!;
  const equipStatus = container.querySelector<HTMLElement>("#gamcha-equip-status")!;
  const alignX = container.querySelector<HTMLInputElement>("#gamcha-align-x")!;
  const alignY = container.querySelector<HTMLInputElement>("#gamcha-align-y")!;
  const alignSize = container.querySelector<HTMLInputElement>("#gamcha-align-size")!;
  const alignXValue = container.querySelector<HTMLOutputElement>("#gamcha-align-x-value")!;
  const alignYValue = container.querySelector<HTMLOutputElement>("#gamcha-align-y-value")!;
  const alignSizeValue = container.querySelector<HTMLOutputElement>("#gamcha-align-size-value")!;
  const alignReset = container.querySelector<HTMLButtonElement>("#gamcha-align-reset")!;
  const orbit = container.querySelector<HTMLElement>(".gamcha-orbit")!;
  const confetti = container.querySelector<HTMLElement>(".gamcha-confetti")!;
  let disposed = false;
  let drawing = false;
  let tickets = 0;
  let currentSnapshot: GamchaSnapshot = {
    tickets: 0,
    totalDraws: 0,
    ownedCount: 0,
    ownedCostumeIds: [],
    equippedCostumeId: null,
    costumeAlignments: {},
  };
  let alignmentSaveTimer: number | undefined;

  const selectedAlignment = (): CostumeAlignment | null => {
    const costume = costumeById.get(outfit.value);
    if (!costume) return null;
    return resolveCostumeAlignment(costume.slot, currentSnapshot.costumeAlignments[costume.id]);
  };

  const renderAlignment = (): void => {
    const alignment = selectedAlignment();
    const disabled = !alignment;
    for (const control of [alignX, alignY, alignSize, alignReset]) control.disabled = disabled;
    if (!alignment) return;
    alignX.value = String(alignment.x);
    alignY.value = String(alignment.y);
    alignSize.value = String(alignment.size);
    alignXValue.value = String(alignment.x);
    alignYValue.value = String(alignment.y);
    alignSizeValue.value = String(alignment.size);
  };

  for (let index = 0; index < 32; index += 1) {
    const orbitItem = document.createElement("img");
    const costume = randomCostume();
    orbitItem.src = costume.url;
    orbitItem.alt = "";
    orbitItem.style.setProperty("--i", String(index));
    orbitItem.style.setProperty("--n", "32");
    orbitItem.style.setProperty("--delay", `${-(index * 0.028125)}s`);
    orbit.append(orbitItem);
  }
  for (let index = 0; index < 56; index += 1) {
    const particle = document.createElement("i");
    particle.style.setProperty("--i", String(index));
    particle.style.setProperty("--hue", String((index * 47) % 360));
    particle.style.setProperty("--x", `${(index * 37) % 100}vw`);
    particle.style.setProperty("--delay", `${-((index * 83) % 1400)}ms`);
    particle.style.setProperty("--drift", `${(index % 2 === 0 ? 1 : -1) * (30 + (index % 7) * 14)}px`);
    confetti.append(particle);
  }

  const renderSnapshot = (snapshot: GamchaSnapshot): void => {
    currentSnapshot = snapshot;
    tickets = snapshot.tickets;
    ticketCount.textContent = String(snapshot.tickets);
    ownedCount.textContent = `COLLECTION ${snapshot.ownedCount} / ${costumes.length}`;
    drawButton.disabled = drawing || snapshot.tickets === 0;
    drawButton.textContent = snapshot.tickets > 0 ? "GAMCHA!" : "집중 완료 티켓 필요";
    const selected = outfit.value || snapshot.equippedCostumeId || "";
    const defaultOption = document.createElement("option");
    defaultOption.value = "";
    defaultOption.textContent = "기본 모습";
    const options = snapshot.ownedCostumeIds.flatMap((id) => {
      const costume = costumeById.get(id);
      if (!costume) return [];
      const option = document.createElement("option");
      option.value = costume.id;
      option.textContent = `${rarityLabel(costume.rarity)} · ${costume.name}`;
      return [option];
    });
    outfit.replaceChildren(defaultOption, ...options);
    outfit.value = options.some((option) => option.value === selected) ? selected : "";
    const equipped = costumeById.get(snapshot.equippedCostumeId ?? "");
    equipStatus.textContent = equipped ? `착용 중 · ${equipped.name}` : "기본 모습";
    renderAlignment();
  };

  const preview = (costume: (typeof costumes)[number]): void => {
    image.hidden = false;
    image.src = costume.url;
    image.alt = costume.name;
    rarity.textContent = rarityLabel(costume.rarity);
    name.textContent = costume.name;
    bubble.dataset.rarity = costume.rarity;
  };

  const draw = async (): Promise<void> => {
    if (drawing || tickets === 0) return;
    drawing = true;
    error.textContent = "";
    newLabel.textContent = "";
    drawButton.disabled = true;
    bubble.classList.remove("revealed");
    bubble.classList.add("spinning");
    try {
      const resultPromise = invoke<GamchaDrawResult>("draw_gamcha");
      for (let frame = 0; frame < 24 && !disposed; frame += 1) {
        preview(randomCostume());
        await wait(rouletteDelay(frame));
      }
      const result = await resultPromise;
      const costume = costumeById.get(result.costumeId);
      if (!costume) throw new Error("missing costume asset");
      preview(costume);
      const ownedCostumeIds = result.isNew
        ? [...currentSnapshot.ownedCostumeIds, result.costumeId]
        : currentSnapshot.ownedCostumeIds;
      renderSnapshot({
        ...result,
        ownedCostumeIds,
        equippedCostumeId: currentSnapshot.equippedCostumeId,
        costumeAlignments: currentSnapshot.costumeAlignments,
      });
      outfit.value = result.costumeId;
      newLabel.textContent = result.isNew ? "NEW! COLLECTION GET" : "DUPLICATE";
      bubble.classList.remove("spinning");
      bubble.classList.add("revealed");
    } catch {
      error.textContent = "추첨하지 못했습니다. 다시 시도해 주세요.";
      bubble.classList.remove("spinning");
      try {
        renderSnapshot(await invoke<GamchaSnapshot>("get_gamcha_state"));
      } catch { /* 다음 열기에서 다시 동기화합니다. */ }
    } finally {
      drawing = false;
      drawButton.disabled = tickets === 0;
    }
  };

  drawButton.addEventListener("click", () => void draw());
  outfit.addEventListener("change", renderAlignment);
  equipButton.addEventListener("click", async () => {
    equipButton.disabled = true;
    equipStatus.textContent = "적용 중";
    try {
      renderSnapshot(
        await invoke<GamchaSnapshot>("equip_gamcha_costume", {
          costumeId: outfit.value || null,
        }),
      );
    } catch {
      equipStatus.textContent = "착용하지 못했습니다";
    } finally {
      equipButton.disabled = false;
    }
  });
  const saveAlignment = (): void => {
    const costumeId = outfit.value;
    if (!costumeId) return;
    const alignment = {
      x: Number(alignX.value),
      y: Number(alignY.value),
      size: Number(alignSize.value),
    };
    alignXValue.value = String(alignment.x);
    alignYValue.value = String(alignment.y);
    alignSizeValue.value = String(alignment.size);
    window.clearTimeout(alignmentSaveTimer);
    alignmentSaveTimer = window.setTimeout(() => {
      void invoke<GamchaSnapshot>("set_gamcha_costume_alignment", {
        costumeId,
        alignment,
      }).then(renderSnapshot).catch(() => {
        equipStatus.textContent = "위치를 저장하지 못했습니다";
      });
    }, 120);
  };
  alignX.addEventListener("input", saveAlignment);
  alignY.addEventListener("input", saveAlignment);
  alignSize.addEventListener("input", saveAlignment);
  alignReset.addEventListener("click", () => {
    const costumeId = outfit.value;
    if (!costumeId) return;
    void invoke<GamchaSnapshot>("set_gamcha_costume_alignment", {
      costumeId,
      alignment: null,
    }).then(renderSnapshot).catch(() => {
      equipStatus.textContent = "위치를 초기화하지 못했습니다";
    });
  });
  container.querySelector<HTMLButtonElement>(".gamcha-close")?.addEventListener("click", () => {
    void invoke("hide_utility_window", { label: "gamcha" });
  });
  renderSnapshot(await invoke<GamchaSnapshot>("get_gamcha_state"));
  const unlisten = await listen<GamchaSnapshot>("gamcha://ticket-earned", ({ payload }) => {
    renderSnapshot(payload);
    bubble.classList.remove("revealed", "spinning");
    rarity.textContent = "TICKET GET!";
    name.textContent = "집중 완료 보상이 도착했습니다";
    newLabel.textContent = "";
  });
  const unlistenEquipped = await listen<GamchaSnapshot>("gamcha://equipped", ({ payload }) => {
    renderSnapshot(payload);
  });
  return () => {
    disposed = true;
    window.clearTimeout(alignmentSaveTimer);
    unlisten();
    unlistenEquipped();
  };
}

export async function mountGamchaNotice(container: HTMLElement): Promise<() => void> {
  container.innerHTML = `
    <main class="gamcha-notice-panel">
      <button class="gamcha-notice-bubble" type="button" aria-label="GAMCHA 보상 열기">
        <span class="gamcha-notice-sparkles" aria-hidden="true">✦ ✧ ✦</span>
        <strong><span>G</span><span>A</span><span>M</span><span>C</span><span>H</span><span>A!</span></strong>
        <small>TICKET <b id="gamcha-notice-tickets">1</b> · 눌러서 뽑기</small>
      </button>
    </main>`;
  const tickets = container.querySelector<HTMLElement>("#gamcha-notice-tickets")!;
  const render = (snapshot: GamchaSnapshot): void => {
    tickets.textContent = String(snapshot.tickets);
  };
  container
    .querySelector<HTMLButtonElement>(".gamcha-notice-bubble")
    ?.addEventListener("click", () => {
      void invoke("show_utility_window", { label: "gamcha" });
    });
  render(await invoke<GamchaSnapshot>("get_gamcha_state"));
  const unlisten = await listen<GamchaSnapshot>("gamcha://ticket-earned", ({ payload }) =>
    render(payload),
  );
  const positionTimer = window.setInterval(() => void invoke("position_gamcha_bubble"), 250);
  return () => {
    window.clearInterval(positionTimer);
    unlisten();
  };
}
