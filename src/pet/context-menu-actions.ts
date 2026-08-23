export type ContextUtilityCommand =
  | { command: "toggle_timer_bubble" }
  | { command: "show_utility_window"; label: string };

export function contextUtilityCommand(action: string): ContextUtilityCommand | null {
  if (action === "timer") return { command: "toggle_timer_bubble" };
  if (["gamcha", "todo", "settings"].includes(action)) {
    return { command: "show_utility_window", label: action };
  }
  return null;
}
export async function showUtilityThenHideMenu(
  label: string,
  showUtility: (label: string) => Promise<void>,
  hideMenu: () => Promise<void>,
): Promise<void> {
  await showUtility(label);
  await hideMenu().catch(() => undefined);
}