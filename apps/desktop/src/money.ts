// Parse decimal text directly to integer minor units; avoid float rounding.
export function parseMoney(value: string, allowZero = false): number | null {
  const match = /^(\d{1,8})(?:[.,](\d{1,2}))?$/.exec(value.trim());
  if (!match) return null;
  const result = Number(match[1]) * 100 + Number((match[2] ?? '').padEnd(2, '0'));
  return result >= (allowZero ? 0 : 1) && result <= 1_000_000_000 ? result : null;
}
export function formatMoney(value: number, language: 'sr' | 'en'): string {
  return new Intl.NumberFormat(language === 'sr' ? 'sr-Latn-RS' : 'en-GB', { style: 'currency', currency: 'RSD', minimumFractionDigits: 2 }).format(value / 100);
}
