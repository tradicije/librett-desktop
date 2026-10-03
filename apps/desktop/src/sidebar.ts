export function savedSidebarCollapsed(): boolean {
  try { return localStorage.getItem('librett.sidebarCollapsed') === 'true'; }
  catch { return false; }
}
export function saveSidebarCollapsed(collapsed: boolean): void {
  try { localStorage.setItem('librett.sidebarCollapsed', String(collapsed)); }
  catch { /* Keep the preference for the current session when storage is unavailable. */ }
}
