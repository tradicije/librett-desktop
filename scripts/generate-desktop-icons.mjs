import { copyFile, mkdir, mkdtemp, readFile, readdir, rm, writeFile } from 'node:fs/promises';
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

function generate(source, output) {
  const result = spawnSync(process.execPath, [cli, 'icon', source, '--output', output], {
    cwd: join(root, 'apps/desktop'),
    stdio: 'inherit',
  });
  if (result.error) throw result.error;
  if (result.status !== 0) throw new Error(`Icon generation failed: ${result.status ?? result.signal}`);
}

try {
  const source = join(root, 'assets/img/app-icon.png');
  generate(source, temporary);

  // Dock icons need breathing room around the artwork. Use an SVG canvas
  // around the original raster so every regeneration uses the same artwork.
  const macSource = join(temporary, 'macos-icon.svg');
  const macOutput = join(temporary, 'macos');
  const png = (await readFile(source)).toString('base64');
  await writeFile(macSource, `<svg xmlns="http://www.w3.org/2000/svg" width="1024" height="1024" viewBox="0 0 1024 1024"><image x="100" y="100" width="824" height="824" href="data:image/png;base64,${png}" /></svg>`);
  generate(macSource, macOutput);

  await mkdir(destination, { recursive: true });
  // This project ships a desktop app; keep generated mobile assets out of it.
  for (const entry of await readdir(temporary, { withFileTypes: true })) {
    if (entry.isFile() && /\.(png|ico|icns)$/.test(entry.name)) {
      const folder = entry.name === 'icon.icns' ? macOutput : temporary;
      await copyFile(join(folder, entry.name), join(destination, entry.name));
    }
  }
  console.log('Linux PNG, Windows ICO/Appx and macOS ICNS icons updated. Fully stop and restart LibreTT to apply them.');
  if (process.platform === 'linux') console.log('For development taskbar icons, also run npm run desktop:install-icon.');
} finally {
  await rm(temporary, { recursive: true, force: true });
}
