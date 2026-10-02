<script lang="ts">
  import { onMount, untrack } from 'svelte';
  import Select from './Select.svelte';
  import { parseMoney, formatMoney } from './money';
  import Icon from './Icon.svelte';
  import { listCash, listEntries, recordCash, desktopAvailable, type Tournament, type Entry, type CashKind, type CashRecord } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let { tournament, language, busy = $bindable(false) }: { tournament: Tournament; language: Language; busy?: boolean } = $props();
  let text = $derived(messages[language]);
  let entries = $state<Entry[]>([]);
  let records = $state<CashRecord[]>([]);
  let loaded = $state(false);
  let loading = $state(false);
  let saving = $state(false);
  let entryId = $state('');
  let kind = $state<CashKind>('payment');
  let amount = $state('');
  let note = $state('');
  let pending = $state<CashRecord | null>(null);
  let error = $state<MessageKey | null>(null);
  let notice = $state(false);
  const kinds: CashKind[] = ['charge', 'discount', 'payment', 'refund'];
  function money(value: number) { return formatMoney(value, language); }
  function label(entry: Entry) {
    const category = tournament.categories.find(c => c.id === entry.category_id);
    const title = `${category?.name ?? ''} · ${text[category?.discipline ?? 'singles']} · ${entry.members.map(m => m.name).join(' / ')}`;
    return title + (category?.archived ? ` (${text.archivedCategory})` : '') + (entry.status === 'withdrawn' ? ` (${text.withdrawnRegistrations})` : '');
  }
  function balance(id?: string) {
    const totals = { charge: 0, discount: 0, payment: 0, refund: 0 };
    for (const r of records) if (!id || r.entry_id === id) totals[r.kind] += r.amount_minor;
    return { due: totals.charge - totals.discount, net: totals.payment - totals.refund, ...totals };
  }
  function suggestedAmount(id: string) {
    const b = balance(id);
    const remaining = Math.max(0, b.due - b.net);
    return remaining ? (remaining / 100).toFixed(2) : '';
  }
  $effect(() => {
    const id = entryId; const transactionKind = kind;
    untrack(() => { if (!pending) { amount = transactionKind === 'payment' ? suggestedAmount(id) : ''; note = ''; } });
  });
  function chooseAccount(id: string) {
    if (busy) return;
    entryId = id; kind = 'payment'; amount = suggestedAmount(id); note = ''; error = null; notice = false;
  }
  let accounts = $derived(entries.map(entry => ({ entry, ...balance(entry.id) })));
  let outstanding = $derived(accounts.reduce((n,a) => n + Math.max(0,a.due-a.net),0));
  let credits = $derived(accounts.reduce((n,a) => n + Math.max(0,a.net-a.due),0));
  let net = $derived(balance().net);
  let selectedRecords = $derived(records.filter(r => r.entry_id === entryId));
  async function load() {
    loading = true; error = null;
    try {
      const [cash, categories] = await Promise.all([listCash(tournament.id), Promise.all(tournament.categories.map(c => listEntries(c.id)))]);
      records = cash; entries = categories.flat(); loaded = true;
      if (!entryId) entryId = entries[0]?.id ?? '';
      if (kind === 'payment') amount = suggestedAmount(entryId);
    } catch (cause) { error = errorKey(cause); }
    finally { loading = false; }
  }
  onMount(() => { if (desktopAvailable) void load(); });
  async function save(event: SubmitEvent) {
    event.preventDefault(); if (saving) return;
    error = null; notice = false;
    if (!pending) {
      const value = parseMoney(amount);
      if (value === null || !entryId || note.trim().length > 500) { error = 'invalid_cash'; return; }
      pending = { id: crypto.randomUUID(), entry_id: entryId, kind, amount_minor: value, note: note.trim(), created_at: '' };
    }
    busy = true; saving = true;
    try {
      await recordCash(tournament.id, pending);
      // Keep the same request if refresh fails after a committed write.
      records = await listCash(tournament.id);
      pending = null; amount = kind === 'payment' ? suggestedAmount(entryId) : ''; note = ''; notice = true; busy = false;
    } catch (cause) {
      error = errorKey(cause);
      if (cause === 'invalid_cash' || cause === 'not_found') { pending = null; busy = false; }
    } finally { saving = false; }
  }
</script>

<div class="heading"><div><p class="eyebrow">{tournament.name}</p><h1>{text.cashDesk}</h1><p class="muted">{text.cashIntro}</p></div></div>
{#if error}<p class="error" role="alert">{text[error]}</p>{/if}
{#if pending && !saving}<p class="banner">{text.cashRetryHint}</p>{/if}
{#if notice}<p class="notice" role="status">{text.cashSaved}</p>{/if}
{#if loading}<p role="status">{text.loading}</p>{/if}
{#if !loaded && desktopAvailable && !loading}<button onclick={load}>{text.retry}</button>{/if}
{#if loaded}
  <div class="cash-totals"><section class="panel"><h2>{text.netReceived}</h2><strong>{money(net)}</strong></section><section class="panel"><h2>{text.outstanding}</h2><strong>{money(outstanding)}</strong></section><section class="panel"><h2>{text.credit}</h2><strong>{money(credits)}</strong></section></div>
  {#if !entries.length}<p class="muted">{text.cashNoEntries}</p>{:else}
    <div class="columns">
      <section class="panel"><h2>{text.cashAccounts}</h2>
        {#each accounts as account (account.entry.id)}
          <button class="tournament cash-account" aria-pressed={entryId === account.entry.id} class:active={entryId === account.entry.id} disabled={busy} onclick={() => chooseAccount(account.entry.id)}><div class="row-content"><strong>{label(account.entry)}</strong><small>{text.amountDue}: {money(account.due)} · {text.netReceived}: {money(account.net)}</small><small>{text.outstanding}: {money(Math.max(0, account.due-account.net))} · {text.credit}: {money(Math.max(0,account.net-account.due))}</small></div><Icon name="arrow-right" /></button>
        {/each}
      </section>
      <section class="panel form-panel"><h2>{text.cashRecord}</h2><form onsubmit={save}>
        <label>{text.cashAccount}<Select label={text.cashAccount} bind:value={entryId} options={entries.map(e => ({ value: e.id, label: label(e) }))} disabled={busy} /></label>
        <label>{text.cashKind}<Select label={text.cashKind} bind:value={kind} options={kinds.map(k => ({ value: k, label: text[k] }))} disabled={busy} /></label>
        <label>{text.cashAmount}<input bind:value={amount} inputmode="decimal" required disabled={busy} placeholder="1000,00" /></label>
        <label>{text.cashNote}<textarea bind:value={note} maxlength="500" disabled={busy}></textarea></label>
        <button class="primary" disabled={saving || !entryId}><Icon name="check-circle" />{saving ? text.saving : pending ? text.retry : text.cashRecord}</button>
      </form></section>
    </div>
    <section class="panel cash-history"><h2>{text.cashHistory}</h2>
      {#if !selectedRecords.length}<p class="muted">{text.cashEmptyHistory}</p>{/if}
      {#each selectedRecords as record (record.id)}<article class="category"><div><h3>{text[record.kind]} · {money(record.amount_minor)}</h3><p>{record.note}</p><small>{new Date(record.created_at).toLocaleString(language === 'sr' ? 'sr-Latn-RS' : 'en-GB')}</small></div></article>{/each}
    </section>
  {/if}
{/if}
