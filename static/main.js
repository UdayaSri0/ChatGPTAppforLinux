import { appWindow } from "@tauri-apps/api/window";
document.getElementById("show")?.addEventListener("click", () => appWindow.show());
