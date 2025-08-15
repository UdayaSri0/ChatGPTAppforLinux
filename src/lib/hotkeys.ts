import type { Config } from './storage';

export function showHotkeys(cfg: Config) {
  const el = document.getElementById('hotkey-info');
  if (el) {
    el.textContent = `Open: ${cfg.hotkeys.openChat} | Prompt: ${cfg.hotkeys.quickPrompt} | Screenshot: ${cfg.hotkeys.screenshot}`;
  }
}
