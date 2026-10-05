import { createHash } from 'node:crypto';
import { createReadStream } from 'node:fs';
import { copyFile, mkdir, readFile, readdir, rename, rm, writeFile } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';
import { randomUUID } from 'node:crypto';
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
const packagedFiles = [];
for (const extension of formats[platform]) {
  const matching = candidates.filter(path => path.endsWith(extension));
  if (matching.length !== 1) {
    throw new Error(`Expected exactly one ${extension} installer in ${input}, found ${matching.length}. Use a clean build directory.`);
  }
  const name = `LibreTT_${metadata.version}_${platform}${extension}`;
  const target = join(destination, name);
  await copyFile(matching[0], target);
  packagedFiles.push(target);
  const hash = createHash('sha256');
  for await (const chunk of createReadStream(target)) hash.update(chunk);
  checksums.push(`${hash.digest('hex')}  ${name}`);
  console.log(target);
}
const checksumFile = join(destination, `SHA256SUMS-${platform}.txt`);
await writeFile(checksumFile, `${checksums.join('\n')}\n`);
packagedFiles.push(checksumFile);

// Plain text instructions open without a Markdown viewer and travel with the installer.
if (platform.startsWith('macos-') || platform.startsWith('windows-')) {
  const os = platform.startsWith('macos-') ? 'macos' : 'windows';
  for (const language of ['sr', 'en']) {
    const source = await readFile(join(root, 'docs/install', `${os}-${language}.txt`), 'utf8');
    const text = source.replaceAll('{{VERSION}}', metadata.version)
      .replaceAll('{{CHECKSUM}}', `SHA256SUMS-${platform}.txt`)
      .replaceAll('{{INSTALLER}}', `LibreTT_${metadata.version}_${platform}${formats[platform][0]}`);
    const path = join(destination, `README-${language.toUpperCase()}.txt`);
    await writeFile(path, text);
    packagedFiles.push(path);
  }
  const archive = join(destination, `LibreTT_${metadata.version}_${platform}.zip`);
  const temporary = join(destination, `.package-${randomUUID()}.zip`);
  try {
    const result = process.platform === 'win32'
      ? spawnSync('powershell.exe', ['-NoProfile', '-NonInteractive', '-Command',
        "$ErrorActionPreference='Stop'; $files=ConvertFrom-Json $env:LIBRETT_ZIP_FILES; Compress-Archive -LiteralPath $files -DestinationPath $env:LIBRETT_ZIP_OUTPUT -CompressionLevel Optimal"],
      { stdio: 'inherit', env: { ...process.env, LIBRETT_ZIP_FILES: JSON.stringify(packagedFiles), LIBRETT_ZIP_OUTPUT: temporary } })
      : spawnSync('zip', ['-j', '-q', temporary, ...packagedFiles], { stdio: 'inherit' });
    if (result.error) throw result.error;
    if (result.status !== 0) throw new Error(`ZIP packaging failed: ${result.status ?? result.signal}`);
    await rename(temporary, archive);
    console.log(archive);
  } finally {
    await rm(temporary, { force: true });
  }
}
