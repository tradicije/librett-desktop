import { createHash } from 'node:crypto';
import { createReadStream } from 'node:fs';
import { copyFile, mkdir, readFile, readdir, writeFile } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const [input, platform] = process.argv.slice(2);
const formats = {
  'macos-arm64': ['.dmg'],
  'macos-x64': ['.dmg'],
  'windows-x64': ['.exe'],
  'linux-x64': ['.deb', '.AppImage'],
};
if (!input || !Object.hasOwn(formats, platform)) {
  throw new Error('Usage: npm run desktop:collect -- <bundle-directory> <macos-arm64|macos-x64|windows-x64|linux-x64>');
}
const metadata = JSON.parse(await readFile(join(root, 'package.json'), 'utf8'));
const desktop = JSON.parse(await readFile(join(root, 'apps/desktop/package.json'), 'utf8'));
const tauri = JSON.parse(await readFile(join(root, 'apps/desktop/src-tauri/tauri.conf.json'), 'utf8'));
const cargo = await readFile(join(root, 'Cargo.toml'), 'utf8');
const cargoVersion = cargo.match(/\[workspace\.package\][\s\S]*?\bversion\s*=\s*"([^"]+)"/)?.[1];
if (![desktop.version, tauri.version, cargoVersion].every(version => version === metadata.version)) {
  throw new Error('Package, Cargo and Tauri versions must match before collecting installers.');
}
const destination = join(root, 'release-assets', platform);
await mkdir(destination, { recursive: true });

async function* files(directory) {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory() && !entry.name.endsWith('.app')) yield* files(path);
    else if (entry.isFile()) yield path;
  }
}
const candidates = [];
for await (const path of files(resolve(input))) candidates.push(path);
const checksums = [];
for (const extension of formats[platform]) {
  const matching = candidates.filter(path => path.endsWith(extension));
  if (matching.length !== 1) {
    throw new Error(`Expected exactly one ${extension} installer in ${input}, found ${matching.length}. Use a clean build directory.`);
  }
  const name = `LibreTT_${metadata.version}_${platform}${extension}`;
  const target = join(destination, name);
  await copyFile(matching[0], target);
  const hash = createHash('sha256');
  for await (const chunk of createReadStream(target)) hash.update(chunk);
  checksums.push(`${hash.digest('hex')}  ${name}`);
  console.log(target);
}
await writeFile(join(destination, `SHA256SUMS-${platform}.txt`), `${checksums.join('\n')}\n`);
