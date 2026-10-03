import { playerLabel } from './player-label';
import type { CategoryDraw, CategoryRules, ScheduledMatch, MatchResult, QualificationSlot } from './api';
import type { Language } from './i18n';
export interface BracketSlot { id?: string; label: string; kind: 'entry' | 'qualifier' | 'bye' | 'pending' | 'winner'; club?: string; seed?: number; group?: number; place?: number }
export function groupName(index: number): string {
  let result = ''; for (let value = index + 1; value > 0; value = Math.floor((value - 1) / 26)) result = String.fromCharCode(65 + (value - 1) % 26) + result;
  return result;
}
export function roundRobin(ids: string[]): [string, string][][] {
  if (ids.length < 2) return [];
  let ring: (string | null)[] = [...ids]; if (ring.length % 2) ring.push(null);
  const rounds: [string, string][][] = [];
  for (let round = 0; round < ring.length - 1; round++) {
    const pairs: [string, string][] = [];
    for (let i = 0; i < ring.length / 2; i++) { const a = ring[i], b = ring[ring.length - 1 - i]; if (a && b) pairs.push(round % 2 ? [b, a] : [a, b]); }
    rounds.push(pairs); ring = [ring[0], ring[ring.length - 1], ...ring.slice(1, -1)];
  }
  return rounds;
}
function seedPositions(size: number): number[] {
  let ranks = [1, 2]; while (ranks.length < size) { const total = ranks.length * 2 + 1; ranks = ranks.flatMap(rank => [rank, total - rank]); }
  return ranks;
}
export function bracketSlots(draw: CategoryDraw | null, rules: CategoryRules, format: 'knockout' | 'groups_knockout', language: Language, qualification?: QualificationSlot[]): BracketSlot[] {
  const empty = (kind: 'bye' | 'pending'): BracketSlot => ({ kind, label: kind === 'bye' ? 'BYE' : language === 'sr' ? 'Neraspoređeno' : 'Unassigned' });
  if (format === 'knockout') {
    if (!draw) return [];
    const complete = new Set(draw.sections.flat().filter(Boolean)).size === draw.participants.length;
    return (draw.sections[0] ?? []).map(id => {
      if (!id) return empty(complete ? 'bye' : 'pending');
      const entry = draw.participants.find(e => e.id === id);
      const seed = draw.seeds.indexOf(id);
      return { id: id!, kind: 'entry', label: entry?.members.map(playerLabel).join(' / ') ?? id, club: [...new Set(entry?.members.map(m => m.club).filter(Boolean))].join(' / '), ...(seed >= 0 ? { seed: seed + 1 } : {}) };
    });
  }
  if (draw && qualification?.length) {
    const entrants = new Map(draw.participants.map(e => [e.id,e]));
    return qualification.map(slot => {
      const entry = slot.entry_id ? entrants.get(slot.entry_id) : undefined;
      if (entry) return { id: entry.id, kind: 'entry', label: entry.members.map(playerLabel).join(' / '), club: [...new Set(entry.members.map(m => m.club).filter(Boolean))].join(' / ') };
      if (slot.bye) return empty('bye');
      return { kind:'qualifier',group:slot.group ?? undefined,place:slot.place ?? undefined,label:language==='sr' ? `${slot.place}. iz grupe ${groupName(slot.group ?? 0)}` : `${groupName(slot.group ?? 0)} · place ${slot.place}` };
    });
  }
  const count = rules.group_count * rules.qualifiers_per_group;
  if (count < 2 || count > 4096) return [];
  const qualified: BracketSlot[] = [];
  for (let place = 1; place <= rules.qualifiers_per_group; place++) for (let group = 0; group < rules.group_count; group++) qualified.push({ kind: 'qualifier', group, place, label: language === 'sr' ? `${place}. iz grupe ${groupName(group)}` : `${groupName(group)} · place ${place}` });
  const size = 2 ** Math.ceil(Math.log2(count));
  const slots = seedPositions(size).map(rank => qualified[rank - 1] ?? empty('bye'));
  // Separate qualifiers from the same group in the opening round where possible.
  // This is a structural preview, never a claim about actual qualified players.
  for (let i = 0; i < slots.length; i += 2) {
    const a = slots[i], b = slots[i + 1];
    if (a.group === undefined || a.group !== b.group) continue;
    const moving = (a.place ?? 0) > (b.place ?? 0) ? i : i + 1;
    const opposite = moving ^ 1;
    const candidate = slots.findIndex((slot, index) => index !== i && index !== i + 1 && slot.group !== undefined && slot.group !== slots[opposite].group && (slot.place ?? 0) > 1 && slots[index ^ 1].group !== slots[moving].group);
    if (candidate >= 0) [slots[moving], slots[candidate]] = [slots[candidate], slots[moving]];
  }
  return slots;
}
export interface BracketMatch { result?: MatchResult; number: number; left: BracketSlot; right: BracketSlot; center: number }
export function bracketRounds(slots: BracketSlot[], language: Language, progress: ScheduledMatch[] = []): BracketMatch[][] {
  if (slots.length < 2 || slots.length > 4096 || !Number.isInteger(Math.log2(slots.length))) return [];
  const saved = new Map(progress.map(match => [match.key, match]));
  let participants = slots; const rounds: BracketMatch[][] = []; let number = 1;
  while (participants.length > 1) {
    const round: BracketMatch[] = []; const next: BracketSlot[] = [];
    for (let i = 0; i < participants.length; i += 2) {
      const left = participants[i], right = participants[i + 1];
      const result = saved.get(`ko:${rounds.length}:${i / 2}`)?.result;
      const current = result && result.first === left.id && result.second === right.id ? result : undefined;
      round.push({ result: current, number, left, right, center: (i / 2 + .5) * 100 * 2 ** rounds.length });
      next.push(current ? (current.winner === left.id ? left : right) : left.kind === 'bye' ? right : right.kind === 'bye' ? left : { kind: 'winner', label: language === 'sr' ? `Pobednik meča ${number}` : `Winner of match ${number}` });
      number++;
    }
    rounds.push(round); participants = next;
  }
  return rounds;
}
export function roundTitle(remaining: number, language: Language): string {
  if (remaining === 2) return language === 'sr' ? 'Finale' : 'Final';
  if (remaining === 4) return language === 'sr' ? 'Polufinale' : 'Semifinals';
  if (remaining === 8) return language === 'sr' ? 'Četvrtfinale' : 'Quarterfinals';
  if (remaining === 32) return language === 'sr' ? 'Šesnaestina finala' : 'Round of 32';
  if (remaining === 16) return language === 'sr' ? 'Osmina finala' : 'Round of 16';
  return language === 'sr' ? `Najboljih ${remaining}` : `Round of ${remaining}`;
}

// Compute a single circle-method round without building the O(n²) schedule.
export function roundRobinRound(ids: string[], round: number): [string, string][] {
  if (ids.length < 2) return [];
  const size = ids.length + ids.length % 2;
  if (round < 0 || round >= size - 1) return [];
  const at = (index: number) => {
    const original = index === 0 ? 0 : 1 + ((index - 1 - round) % (size - 1) + size - 1) % (size - 1);
    return ids[original] ?? null;
  };
  const pairs: [string, string][] = [];
  for (let i = 0; i < size / 2; i++) { const a = at(i), b = at(size - 1 - i); if (a && b) pairs.push(round % 2 ? [b, a] : [a, b]); }
  return pairs;
}
