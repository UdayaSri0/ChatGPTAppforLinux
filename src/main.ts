import { writeText } from '@tauri-apps/api/clipboard';
import { invoke } from '@tauri-apps/api/tauri';
import Fuse from 'fuse.js';
import { loadSnippets, loadConfig, Snippet } from './lib/storage';
import { showHotkeys } from './lib/hotkeys';

let snippets: Snippet[] = [];
let fuse: Fuse<Snippet>;

const input = document.getElementById('prompt') as HTMLInputElement;
const list = document.getElementById('snippets') as HTMLUListElement;

function render(items: Snippet[]) {
  list.innerHTML = '';
  items.slice(0, 5).forEach(sn => {
    const li = document.createElement('li');
    li.textContent = sn.title;
    li.onclick = async () => {
      await writeText(sn.body);
      window.close();
    };
    list.appendChild(li);
  });
}

async function init() {
  try {
    const cfg = await loadConfig();
    showHotkeys(cfg);
    snippets = await loadSnippets();
    fuse = new Fuse(snippets, { keys: ['title', 'body'] });
    render(snippets);
  } catch (err) {
    console.error('Initialization failed', err);
    alert('Failed to load configuration. Check console for details.');
  }
}
init();

input.addEventListener('input', () => {
  const term = input.value.trim();
  if (!term) {
    render(snippets);
  } else {
    render(fuse.search(term).map(r => r.item));
  }
});

document.getElementById('copy')!.addEventListener('click', async () => {
  await writeText(input.value);
  window.close();
});

document.getElementById('copyFocus')!.addEventListener('click', async () => {
  await writeText(input.value);
  await invoke('open_chatgpt');
  window.close();
});
