import placeholderSprite from "../images/app/character/idle.svg";
import "./styles.css";

const app = document.querySelector<HTMLDivElement>("#app");

if (!app) {
  throw new Error("The application root is missing.");
}

app.innerHTML = `
  <main class="pet-shell" aria-label="Desktop pet placeholder">
    <img class="pet-placeholder" src="${placeholderSprite}" alt="Temporary geometric desktop pet" />
  </main>
`;
