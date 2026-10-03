/** Display-only labels: stored names and club values remain unchanged. */
export function clubCode(club: string): string {
  return Array.from(club.trim().normalize('NFC').replace(/[^\p{L}\p{N}]/gu, '').toLocaleUpperCase('sr-Latn')).slice(0, 3).join('');
}
export function playerLabel(player: { name: string; club: string } | null | undefined): string {
  if (!player) return '';
  const code = clubCode(player.club);
  return code ? `${player.name} (${code})` : player.name;
}
