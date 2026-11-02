export type ThemePreference = 'light' | 'dark' | 'system';
export type ResolvedTheme = 'light' | 'dark';

export function savedTheme(): ThemePreference {
  try {
    const value = localStorage.getItem('librett.theme');
    if (value === 'light' || value === 'dark') return value;
  } catch { /* Preferences are optional. */ }
  return 'system';
}

export function resolveTheme(preference: ThemePreference, systemDark: boolean): ResolvedTheme {
  return preference === 'system' ? (systemDark ? 'dark' : 'light') : preference;
}

export function applyTheme(preference: ThemePreference): ResolvedTheme {
  const resolved = resolveTheme(preference, window.matchMedia('(prefers-color-scheme: dark)').matches);
  document.documentElement.dataset.theme = resolved;
  document.documentElement.style.colorScheme = resolved;
  return resolved;
}

export function saveTheme(preference: ThemePreference): ResolvedTheme {
  try { localStorage.setItem('librett.theme', preference); } catch { /* Keep working without storage. */ }
  return applyTheme(preference);
}

export function watchSystemTheme(onChange: () => void): () => void {
  const media = window.matchMedia('(prefers-color-scheme: dark)');
  media.addEventListener('change', onChange);
  return () => media.removeEventListener('change', onChange);
}
