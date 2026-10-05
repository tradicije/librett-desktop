import { copyFile, mkdir, readFile, writeFile } from 'node:fs/promises';
import { homedir } from 'node:os';
import { dirname, isAbsolute, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

// Dev binaries have no installer to register their icon with the desktop shell.
if (process.platform !== 'linux') throw new Error('This command is for Linux development only.');
if (process.argv.slice(2).some(arg => arg !== '--no-refresh')) throw new Error('Supported option: --no-refresh');
const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const config = JSON.parse(await readFile(join(root, 'apps/desktop/src-tauri/tauri.conf.json'), 'utf8'));
const id = config.identifier;
if (!/^[a-zA-Z][a-zA-Z0-9.-]+$/.test(id)) throw new Error('Invalid desktop application ID.');
const dataRoot = process.env.XDG_DATA_HOME && isAbsolute(process.env.XDG_DATA_HOME)
  ? process.env.XDG_DATA_HOME : join(homedir(), '.local/share');
const entry = join(dataRoot, 'applications', `${id}.desktop`);
const marker = '# LibreTT development icon registration';
try {
  const existing = await readFile(entry, 'utf8');
  if (!existing.startsWith(`${marker}\n`)) throw new Error(`Existing desktop entry left untouched: ${entry}`);
} catch (error) {
  if (error.code !== 'ENOENT') throw error;
}
for (const size of [32, 128, 256]) {
  const source = size === 256 ? '128x128@2x.png' : `${size}x${size}.png`;
  const destination = join(dataRoot, 'icons/hicolor', `${size}x${size}`, 'apps', `${id}.png`);
  await mkdir(dirname(destination), { recursive: true });
  await copyFile(join(root, 'apps/desktop/src-tauri/icons', source), destination);
}
await mkdir(dirname(entry), { recursive: true });
// Hidden metadata for matching running windows, not a development launcher:
// the dev binary needs the Vite server started by `npm run desktop -- dev`.
await writeFile(entry, `${marker}\n[Desktop Entry]\nType=Application\nName=LibreTT\nComment=LibreTT development window identity\nExec=LibreTT\nIcon=${id}\nStartupWMClass=${id}\nNoDisplay=true\nTerminal=false\nCategories=Utility;\n`, { mode: 0o644 });
console.log(`Registered development icon: ${entry}`);
if (!process.argv.includes('--no-refresh')) {
  const refresh = spawnSync('kbuildsycoca6', [], { encoding: 'utf8' });
  if ((refresh.error && refresh.error.code !== 'ENOENT') || (refresh.status !== null && refresh.status !== 0))
    console.warn('Desktop cache refresh failed; log out and back in if the icon remains stale.');
}
console.log('Stop LibreTT and restart npm run desktop -- dev to use the GTK application ID.');
