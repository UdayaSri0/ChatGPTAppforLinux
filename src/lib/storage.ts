import { BaseDirectory, readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import { appDir, resolveResource } from '@tauri-apps/api/path';

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

async function ensureFile(name: string) {
  try {
    await readTextFile(name, { dir: BaseDirectory.App });
  } catch (_) {
    const res = await resolveResource(name);
    const data = await readTextFile(res);
    await writeTextFile({ path: name, contents: data, dir: BaseDirectory.App });
  }
}

export async function loadConfig(): Promise<Config> {
  await ensureFile('config.json');
  const data = await readTextFile('config.json', { dir: BaseDirectory.App });
  return JSON.parse(data) as Config;
}

export async function loadSnippets(): Promise<Snippet[]> {
  await ensureFile('snippets.json');
  const data = await readTextFile('snippets.json', { dir: BaseDirectory.App });
  return JSON.parse(data) as Snippet[];
}

export async function saveSnippets(list: Snippet[]): Promise<void> {
  await writeTextFile({ path: 'snippets.json', contents: JSON.stringify(list, null, 2), dir: BaseDirectory.App });
}
