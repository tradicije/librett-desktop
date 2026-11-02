let ask: (() => Promise<boolean>) | undefined;
export function registerDiscardConfirmation(handler: () => Promise<boolean>) {
  ask = handler;
  return () => { if (ask === handler) ask = undefined; };
}
export function confirmDiscard(): Promise<boolean> { return ask?.() ?? Promise.resolve(false); }
