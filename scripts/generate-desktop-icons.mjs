import { copyFile, mkdir, mkdtemp, readdir, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const require = createRequire(import.meta.url);
const cli = require.resolve('@tauri-apps/cli/tauri.js');
const destination = join(root, 'apps/desktop/src-tauri/icons');
const temporary = await mkdtemp(join(tmpdir(), 'librett-icons-'));

try {
  const result = spawnSync(process.execPath, [cli, 'icon', join(root, 'assets/img/app-icon.png'), '--output', temporary], {
    cwd: join(root, 'apps/desktop'),
    stdio: 'inherit',
  });
  if (result.error) throw result.error;
  if (result.status !== 0) throw new Error(`Icon generation failed: ${result.status ?? result.signal}`);
  await mkdir(destination, { recursive: true });
  // This project ships a desktop app; keep generated mobile assets out of it.
  for (const entry of await readdir(temporary, { withFileTypes: true })) {
    if (entry.isFile() && /\.(png|ico|icns)$/.test(entry.name)) {
      await copyFile(join(temporary, entry.name), join(destination, entry.name));
    }
  }
  console.log('Linux PNG, Windows ICO/Appx and macOS ICNS icons updated. Fully stop and restart LibreTT to apply them.');
  if (process.platform === 'linux') console.log('For development taskbar icons, also run npm run desktop:install-icon.');
} finally {
  await rm(temporary, { recursive: true, force: true });
}
