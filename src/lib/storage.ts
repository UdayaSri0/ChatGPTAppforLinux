import { BaseDirectory, readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import { resolveResource } from '@tauri-apps/api/path';

export interface Hotkeys {
  openChat: string;
  quickPrompt: string;
  screenshot: string;
}

export interface Config {
  chatUrl: string;
  hotkeys: Hotkeys;
  browserCandidates: string[];
}

export interface Snippet {
  title: string;
  body: string;
}

function defaultConfig(): Config {
  return {
    chatUrl: 'https://chatgpt.com',
    hotkeys: {
      openChat: 'Ctrl+Space',
      quickPrompt: 'Ctrl+Shift+P',
      screenshot: 'Ctrl+Shift+S',
    },
    browserCandidates: ['google-chrome', 'chromium', 'brave-browser'],
  };
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null;
}

function normalizeConfig(value: unknown): Config {
  const defaults = defaultConfig();
  if (!isRecord(value)) {
    return defaults;
  }

  const hotkeys = isRecord(value.hotkeys) ? value.hotkeys : {};
  const browserCandidates = Array.isArray(value.browserCandidates)
    ? value.browserCandidates.filter(
        (candidate): candidate is string =>
          typeof candidate === 'string' && candidate.trim().length > 0
      )
    : [];

  return {
    chatUrl:
      typeof value.chatUrl === 'string' && value.chatUrl.trim().length > 0
        ? value.chatUrl
        : defaults.chatUrl,
    hotkeys: {
      openChat:
        typeof hotkeys.openChat === 'string' && hotkeys.openChat.trim().length > 0
          ? hotkeys.openChat
          : defaults.hotkeys.openChat,
      quickPrompt:
        typeof hotkeys.quickPrompt === 'string' && hotkeys.quickPrompt.trim().length > 0
          ? hotkeys.quickPrompt
          : defaults.hotkeys.quickPrompt,
      screenshot:
        typeof hotkeys.screenshot === 'string' && hotkeys.screenshot.trim().length > 0
          ? hotkeys.screenshot
          : defaults.hotkeys.screenshot,
    },
    browserCandidates:
      browserCandidates.length > 0 ? browserCandidates : defaults.browserCandidates,
  };
}

function normalizeSnippets(value: unknown): Snippet[] {
  if (!Array.isArray(value)) {
    return [];
  }
  return value
    .filter(
      (item): item is { title: string; body: string } =>
        isRecord(item) && typeof item.title === 'string' && typeof item.body === 'string'
    )
    .map(item => ({ title: item.title, body: item.body }));
}

async function ensureFile(name: string) {
  try {
    await readTextFile(name, { dir: BaseDirectory.App });
  } catch (_) {
    try {
      const res = await resolveResource(name);
      const data = await readTextFile(res);
      await writeTextFile({ path: name, contents: data }, { dir: BaseDirectory.App });
    } catch (err) {
      console.error(`Failed to initialize ${name}`, err);
    }
  }
}

export async function loadConfig(): Promise<Config> {
  await ensureFile('config.json');
  try {
    const data = await readTextFile('config.json', { dir: BaseDirectory.App });
    return normalizeConfig(JSON.parse(data));
  } catch (err) {
    console.error('Failed to load config', err);
    return defaultConfig();
  }
}

export async function loadSnippets(): Promise<Snippet[]> {
  await ensureFile('snippets.json');
  try {
    const data = await readTextFile('snippets.json', { dir: BaseDirectory.App });
    return normalizeSnippets(JSON.parse(data));
  } catch (err) {
    console.error('Failed to load snippets', err);
    return [];
  }
}

export async function saveSnippets(list: Snippet[]): Promise<void> {
  try {
    await writeTextFile(
      { path: 'snippets.json', contents: JSON.stringify(list, null, 2) },
      { dir: BaseDirectory.App }
    );
  } catch (err) {
    console.error('Failed to save snippets', err);
    throw err;
  }
}
