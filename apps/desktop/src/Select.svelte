<script lang="ts" generics="T extends string">
  import { tick } from 'svelte';
  import { IconChevronDown } from '@tabler/icons-svelte';
  let { value = $bindable(), options, label, placeholder = '', disabled = false }: {
    value: T; options: { value: T; label: string }[]; label: string;
    placeholder?: string; disabled?: boolean;
  } = $props();
  const id = $props.id();
  let root: HTMLDivElement;
  let trigger: HTMLButtonElement;
  let open = $state(false);
  let active = $state(0);
  let query = '';
  let lastTyped = 0;
  const selected = $derived(options.find(option => option.value === value));
  $effect(() => { if (disabled) open = false; });
  async function reveal(index: number) {
    active = Math.max(0, Math.min(options.length - 1, index));
    await tick();
    document.getElementById(`${id}-${active}`)?.scrollIntoView({ block: 'nearest' });
  }
  function show() {
    if (disabled || !options.length) return;
    open = true;
    void reveal(Math.max(0, options.findIndex(option => option.value === value)));
  }
  function choose(index: number) {
    if (!options[index]) return;
    value = options[index].value;
    open = false;
    trigger.focus();
  }
  function keydown(event: KeyboardEvent) {
    if (event.key === 'Tab' || event.key === 'Escape') { open = false; return; }
    if (['ArrowDown', 'ArrowUp', 'Home', 'End', 'Enter', ' '].includes(event.key)) {
      event.preventDefault();
      if (!open) { show(); return; }
      if (event.key === 'Enter' || event.key === ' ') choose(active);
      else void reveal(event.key === 'Home' ? 0 : event.key === 'End' ? options.length - 1 : active + (event.key === 'ArrowDown' ? 1 : -1));
    } else if (event.key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey) {
      event.preventDefault();
      const now = Date.now();
      query = (now - lastTyped < 600 ? query : '') + event.key.toLocaleLowerCase();
      lastTyped = now;
      if (!open) show();
      const index = options.findIndex(option => option.label.toLocaleLowerCase().startsWith(query));
      if (index >= 0) void reveal(index);
    }
  }
</script>

<svelte:window onpointerdown={(event) => { if (root && !root.contains(event.target as Node)) open = false; }} />
<div class="select-control" bind:this={root} onfocusout={(event) => { if (!root.contains(event.relatedTarget as Node)) open = false; }}>
  <button type="button" class="select-trigger" bind:this={trigger} role="combobox"
    aria-label={label} aria-expanded={open} aria-haspopup="listbox" aria-controls={id}
    aria-activedescendant={open ? `${id}-${active}` : undefined}
    disabled={disabled || !options.length} onkeydown={keydown} onclick={() => open ? open = false : show()}>
    <span>{selected?.label ?? placeholder}</span><IconChevronDown size={18} stroke={1.75} aria-hidden="true" />
  </button>
  {#if open}
    <div class="select-menu" id={id} role="listbox" aria-label={label} tabindex="-1">
      {#each options as option, index (option.value)}
        <button type="button" id={`${id}-${index}`} role="option" tabindex="-1"
          aria-selected={option.value === value} class:highlighted={index === active}
          onpointerdown={(event) => event.preventDefault()} onclick={() => choose(index)}>{option.label}</button>
      {/each}
    </div>
  {/if}
</div>
