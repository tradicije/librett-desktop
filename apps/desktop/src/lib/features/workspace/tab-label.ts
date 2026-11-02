/** Apply a fade only when the label actually overflows its available width. */
export function fadeOverflow(node: HTMLElement) {
  let mounted = true;
  const measure = () => node.classList.toggle('overflowing', node.scrollWidth > node.clientWidth);
  const resize = new ResizeObserver(measure);
  const text = new MutationObserver(measure);
  resize.observe(node);
  text.observe(node, { childList: true, characterData: true, subtree: true });
  measure();
  void document.fonts.ready.then(() => { if (mounted) measure(); });
  return { destroy() { mounted = false; resize.disconnect(); text.disconnect(); } };
}
