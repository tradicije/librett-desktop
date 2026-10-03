<script lang="ts">
  import PlayerName from './PlayerName.svelte';
  import { playerLabel } from './player-label';
  import { onMount, tick, untrack } from 'svelte';
  import Icon from './Icon.svelte';
  import { formatMoney } from './money';
  import { cashLedger, listEntries, settlePlayerCash, desktopAvailable, type Tournament, type Entry, type CashLedger, type ExpectedCashAmount } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let { active = true, tournament, language, busy = $bindable(false) }: { active?: boolean; tournament: Tournament; language: Language; busy?: boolean } = $props();
  const uid = $props.id();
  let text = $derived(messages[language]);
  let entries = $state<Entry[]>([]);
  let ledger = $state<CashLedger>({ records: [], allocations: [] });
  let loading = $state(false);
  let loaded = $state(false);
  let saving = $state(false);
  let search = $state('');
  let selected = $state<Record<string, string[]>>({});
  let error = $state<MessageKey | null>(null);
  let notice = $state(false);
  type Request = { id: string; playerId: string; entryIds: string[]; paid: boolean; expected: ExpectedCashAmount[] };
  let pending = $state<Request | null>(null);
  let refund = $state<{ playerId: string; name: string; accounts: { entryId: string; label: string; amount: number }[] } | null>(null);
  let dialog: HTMLDialogElement;
  function money(amount: number) { return formatMoney(amount, language); }
  function share(amount: number, position: number, count: number) { return Math.trunc(amount / count) + (position < Math.abs(amount % count) ? Math.sign(amount) : 0); }
  function entryBalance(entry: Entry, playerId?: string) {
    let due = 0; let net = 0; let allocatedNet = 0;
    for (const record of ledger.records.filter(r => r.entry_id === entry.id)) {
      if (record.kind === 'charge') due += record.amount_minor;
      if (record.kind === 'discount') due -= record.amount_minor;
      const sign = record.kind === 'payment' ? 1 : record.kind === 'refund' ? -1 : 0;
      const allocations = ledger.allocations.filter(a => a.record_id === record.id);
      if (playerId && allocations.length) allocatedNet += sign * allocations.filter(a => a.player_id === playerId).reduce((sum,a) => sum+a.amount_minor,0);
      else net += sign * record.amount_minor;
    }
    if (playerId) { const position = entry.members.findIndex(m => m.id === playerId); due = share(due,position,entry.members.length); net = share(net,position,entry.members.length)+allocatedNet; }
    return { due, net, remaining: Math.max(0,due-net) };
  }
  function categoryLabel(entry: Entry) {
    const category = tournament.categories.find(c => c.id === entry.category_id);
    return `${category?.name ?? ''} · ${text[category?.discipline ?? 'singles']}`;
  }
  let rows = $derived.by(() => {
    const players = new Map<string, { id: string; name: string; club: string; entries: Entry[] }>();
    for (const entry of entries) for (const member of entry.members) {
      let player = players.get(member.id);
      if (!player) { player = { id: member.id, name: member.name, club: member.club, entries: [] }; players.set(member.id,player); }
      player.entries.push(entry);
    }
    return [...players.values()].map(player => {
      const accounts = player.entries.map(entry => ({ entry, ...entryBalance(entry,player.id) }));
      const chosen = accounts.filter(a => selected[player.id]?.includes(a.entry.id));
      return { ...player, accounts, remaining: accounts.reduce((sum,a) => sum+a.remaining,0), selectedRemaining: chosen.reduce((sum,a) => sum+a.remaining,0), selectedNet: chosen.reduce((sum,a) => sum+Math.max(0,a.net),0) };
    }).sort((a,b) => a.name.localeCompare(b.name,language));
  });
  let filtered = $derived(rows.filter(row => `${playerLabel(row)} ${row.club} ${row.entries.map(categoryLabel).join(' ')}`.toLocaleLowerCase().includes(search.trim().toLocaleLowerCase())));
  let cashCategories = $derived(tournament.categories.filter(category => entries.some(entry => entry.category_id === category.id)));
  let outstanding = $derived(entries.reduce((sum,e) => sum+entryBalance(e).remaining,0));
  let netReceived = $derived(ledger.records.reduce((sum,r) => sum+(r.kind === 'payment' ? r.amount_minor : r.kind === 'refund' ? -r.amount_minor : 0),0));
  let registered = $derived(new Set(entries.filter(e => e.status !== 'withdrawn' && !tournament.categories.find(c => c.id === e.category_id)?.archived).flatMap(e => e.members.map(m => m.id))).size);
  async function load(preserveSelection = false) {
    loading = true; error = null;
    try {
      const [cash, groups] = await Promise.all([cashLedger(tournament.id), Promise.all(tournament.categories.map(c => listEntries(c.id)))]);
      ledger = cash; entries = groups.flat();
      selected = preserveSelection ? Object.fromEntries(Object.entries(selected).map(([player, ids]) => [player, ids.filter(id => entries.some(entry => entry.id === id))])) : {};
      loaded = true;
    } catch (cause) { error = errorKey(cause); }
    finally { loading = false; }
  }
  onMount(() => { if (desktopAvailable) void load(); });
  let wasActive = untrack(() => active);
  $effect(() => {
    if (active && !wasActive && desktopAvailable && !busy) void load(true);
    wasActive = active;
  });
  function toggleCategory(playerId: string, entryId: string) {
    if (busy) return;
    const choices = selected[playerId] ?? [];
    selected = { ...selected, [playerId]: choices.includes(entryId) ? choices.filter(id => id !== entryId) : [...choices,entryId] };
    notice = false;
  }
  async function paySelected(row: typeof rows[number]) {
    if (busy || loading || row.selectedRemaining <= 0) return;
    const entryIds = row.accounts.filter(account => account.remaining > 0 && selected[row.id]?.includes(account.entry.id)).map(account => account.entry.id);
    pending = { id: crypto.randomUUID(), playerId: row.id, entryIds, paid: true, expected: row.accounts.filter(account => entryIds.includes(account.entry.id)).map(account => ({ entry_id: account.entry.id, amount_minor: account.remaining })) };
    await save();
  }
  async function refundSelected(row: typeof rows[number]) {
    if (busy || loading || row.selectedNet <= 0) return;
    refund = { playerId: row.id, name: playerLabel(row), accounts: row.accounts.filter(account => account.net > 0 && selected[row.id]?.includes(account.entry.id)).map(account => ({ entryId: account.entry.id, label: categoryLabel(account.entry), amount: account.net })) };
    await tick(); dialog.showModal();
  }
  async function confirmRefund() {
    if (!refund || busy) return;
    pending = { id: crypto.randomUUID(), playerId: refund.playerId, entryIds: refund.accounts.map(account => account.entryId), paid: false, expected: refund.accounts.map(account => ({ entry_id: account.entryId, amount_minor: account.amount })) };
    dialog.close(); await save();
  }
  async function save() {
    if (!pending || saving) return;
    busy = true; saving = true; error = null; notice = false;
    try {
      await settlePlayerCash(pending.id,tournament.id,pending.playerId,pending.entryIds,pending.paid,pending.expected);
      ledger = await cashLedger(tournament.id);
      selected = { ...selected, [pending.playerId]: [] };
      pending = null; busy = false; notice = true;
    } catch (cause) {
      error = errorKey(cause);
      if (cause === 'cash_conflict') { pending = null; busy = false; await load(true); error = 'cash_conflict'; }
      if (cause === 'invalid_cash' || cause === 'not_found') { pending = null; busy = false; }
    } finally { saving = false; }
  }
</script>

<div class="heading"><div><p class="eyebrow">{tournament.name}</p><h1>{text.cashDesk}</h1><p class="muted">{text.playerCashIntro}</p></div></div>
{#if error}<p class="error" role="alert">{text[error]}</p>{/if}
{#if pending && !saving}<p class="banner">{text.cashRetryHint} <button class="secondary" onclick={save}>{text.retry}</button></p>{/if}
{#if notice}<p class="notice" role="status">{text.cashSaved}</p>{/if}
{#if loading}<p role="status">{text.loading}</p>{/if}
{#if !loaded && desktopAvailable && !loading}<button onclick={() => load()}>{text.retry}</button>{/if}
{#if loaded}
  <div class="cash-totals"><section class="panel"><h2>{text.outstanding}</h2><strong>{money(outstanding)}</strong></section><section class="panel"><h2>{text.netReceived}</h2><strong>{money(netReceived)}</strong></section><section class="panel"><h2>{text.registeredPlayers}</h2><strong>{registered}</strong></section></div>
  <section class="panel player-cash-list">
    <div class="directory-toolbar"><label class="search-field"><Icon name="search" size={17} /><input type="search" aria-label={text.searchCashPlayers} placeholder={text.searchCashPlayers} bind:value={search} disabled={busy || loading} /></label><span class="muted">{filtered.length} / {rows.length}</span></div>
    <!-- svelte-ignore a11y_no_noninteractive_tabindex (Keyboard users need a focusable container to scroll the category matrix horizontally.) -->
    <div class="cash-table-scroll" role="region" aria-label={text.cashDesk} tabindex="0">
      <table class="cash-player-table" style:min-width={`${380 + cashCategories.length * 170}px`}>
        <thead><tr><th scope="col">{text.firstPlayer}</th>{#each cashCategories as category (category.id)}<th scope="col">{category.name}<small>{text[category.discipline]}{category.archived ? ` · ${text.archivedCategory}` : ''}</small></th>{/each}<th scope="col">{text.outstanding}</th><th scope="col">{text.cashActions}</th></tr></thead>
        <tbody>
          {#each filtered as row (row.id)}
            <tr class="cash-player-row" class:is-paid={row.remaining === 0}>
              <th scope="row"><div class="cash-player-identity"><span class="player-avatar" aria-hidden="true">{row.name.split(/\s+/).slice(0,2).map(part => part[0]).join('').toLocaleUpperCase()}</span><div><h3><PlayerName player={row} /></h3><p>{row.club}</p></div></div></th>
              {#each cashCategories as category (category.id)}
                {@const account = row.accounts.find(a => a.entry.category_id === category.id)}
                <td>
                  {#if account}
                    <label class="cash-category-choice"><input type="checkbox" checked={selected[row.id]?.includes(account.entry.id)} onchange={() => toggleCategory(row.id,account.entry.id)} disabled={busy || loading || (account.remaining === 0 && account.net <= 0)} aria-label={`${text.selectCashCategory}: ${playerLabel(row)} · ${categoryLabel(account.entry)}`} /><span class="cash-category-amount">{account.remaining === 0 ? text.paid : money(account.remaining)}</span></label>
                    {#if account.entry.members.length === 2}<small class="cash-cell-detail"><PlayerName player={account.entry.members.find(m => m.id !== row.id)} /></small>{/if}
                    {#if account.entry.status === 'withdrawn'}<small class="cash-cell-detail">{text.withdrawnRegistrations}</small>{/if}
                  {:else}<span class="cash-cell-empty" aria-label={text.notRegisteredCategory}>—</span>{/if}
                </td>
              {/each}
              <td class="cash-player-total"><strong>{money(row.remaining)}</strong>{#if row.selectedRemaining !== row.remaining}<small>{text.selectedCashAmount}: {money(row.selectedRemaining)}</small>{/if}</td>
              <td><div class="cash-row-actions"><button class="primary cash-pay-button" onclick={() => paySelected(row)} disabled={busy || loading || row.selectedRemaining <= 0} aria-label={`${text.pay}: ${playerLabel(row)}`}>{text.pay}</button><button class="secondary" onclick={() => refundSelected(row)} disabled={busy || loading || row.selectedNet <= 0} aria-label={`${text.refund}: ${playerLabel(row)}`}>{text.refund}</button></div></td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    {#if !filtered.length}<p class="muted">{rows.length ? text.noPlayers : text.cashNoEntries}</p>{/if}
  </section>
{/if}
<dialog class="confirm-dialog" bind:this={dialog} aria-labelledby={`${uid}-cash-refund-title`} onclose={() => { refund = null; }}>
  <h2 id={`${uid}-cash-refund-title`}>{text.confirmCashRefund}</h2><p><PlayerName label={refund?.name ?? ''} /></p>
  <ul class="cash-refund-items">{#each refund?.accounts ?? [] as account (account.entryId)}<li><span>{account.label}</span><strong>{money(account.amount)}</strong></li>{/each}</ul>
  <p><strong>{text.refund}: {money(refund?.accounts.reduce((sum,account) => sum+account.amount,0) ?? 0)}</strong></p>
  <p>{text.cashRefundHint}</p><div class="dialog-actions"><button class="secondary" onclick={() => dialog.close()}>{text.cancelDelete}</button><button class="primary" onclick={confirmRefund}>{text.refund}</button></div>
</dialog>
