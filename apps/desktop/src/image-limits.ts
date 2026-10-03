// Read encoded dimensions before handing an image to the browser decoder.
export function imageDimensions(bytes: Uint8Array, mime: string): { width: number; height: number } {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const ascii = (start: number, count: number) => String.fromCharCode(...bytes.slice(start, start + count));
  if (mime === 'image/png' && bytes.length >= 33 && bytes[0] === 137 && ascii(1, 3) === 'PNG' && ascii(12, 4) === 'IHDR') return { width: view.getUint32(16), height: view.getUint32(20) };
  if (mime === 'image/jpeg' && bytes[0] === 255 && bytes[1] === 216) {
    let offset = 2; let dimensions: { width: number; height: number } | undefined;
    while (offset + 4 <= bytes.length) {
      if (bytes[offset++] !== 255) throw new Error('Invalid JPEG');
      while (bytes[offset] === 255) offset++;
      const marker = bytes[offset++];
      if (marker === 217 || marker === 218) { if (dimensions) return dimensions; break; }
      if (marker === 1 || marker >= 208 && marker <= 215) continue;
      if (offset + 2 > bytes.length) break;
      const length = view.getUint16(offset);
      if (length < 2 || offset + length > bytes.length) break;
      if ([192, 193, 194, 195, 197, 198, 199, 201, 202, 203, 205, 206, 207].includes(marker) && length >= 8) { if (dimensions) throw new Error('Multiple JPEG frames'); dimensions = { width: view.getUint16(offset + 5), height: view.getUint16(offset + 3) }; }
      offset += length;
    }
  }
  if (mime === 'image/webp' && bytes.length >= 30 && ascii(0, 4) === 'RIFF' && ascii(8, 4) === 'WEBP') {
    // Animated and extended WebP may contain additional frames: reject them.
    if (ascii(12, 4) === 'VP8 ' && bytes[23] === 157 && bytes[24] === 1 && bytes[25] === 42) return { width: view.getUint16(26, true) & 16383, height: view.getUint16(28, true) & 16383 };
    if (ascii(12, 4) === 'VP8L' && bytes[20] === 47) { const bits = view.getUint32(21, true); return { width: (bits & 16383) + 1, height: ((bits >>> 14) & 16383) + 1 }; }
  }
  throw new Error('Unsupported image');
}
export async function readBoundedImage(file: File): Promise<HTMLImageElement> {
  if (!['image/jpeg', 'image/png', 'image/webp'].includes(file.type) || file.size > 10 * 1024 * 1024) throw new Error('Image limit');
  const bytes = new Uint8Array(await file.arrayBuffer());
  const dimensions = imageDimensions(bytes, file.type);
  if (!dimensions.width || !dimensions.height || dimensions.width > 6000 || dimensions.height > 6000 || dimensions.width * dimensions.height > 16_000_000) throw new Error('Image dimensions');
  if (file.type === 'image/png') {
    const view = new DataView(bytes.buffer);
    for (let offset = 8; offset + 12 <= bytes.length;) {
      const length = view.getUint32(offset); if (offset + length + 12 > bytes.length) throw new Error('Invalid PNG');
      if (String.fromCharCode(...bytes.slice(offset + 4, offset + 8)) === 'acTL') throw new Error('Animated PNG');
      offset += length + 12;
    }
  }
  const url = URL.createObjectURL(file);
  try {
    const image = new Image();
    await new Promise<void>((resolve, reject) => { image.onload = () => resolve(); image.onerror = reject; image.src = url; });
    if (image.naturalWidth !== dimensions.width || image.naturalHeight !== dimensions.height) throw new Error('Unexpected image dimensions');
    return image;
  } finally { URL.revokeObjectURL(url); }
}
