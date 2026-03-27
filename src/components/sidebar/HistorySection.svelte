<script lang="ts">
  import { appStore } from "../../lib/stores/app.svelte";
  import { editorStore } from "../../lib/stores/editor.svelte";
  import { historyStore } from "../../lib/stores/history.svelte";
  import { createEmptyPair, createEmptyBody, type RequestHistoryEntry } from "../../lib/types";
  import HttpMethodBadge from "../shared/HttpMethodBadge.svelte";

  let expanded = $state(false);

  const recentHistory = $derived(historyStore.history.slice(0, 10));

  function findSavedRequest(requestId: string) {
    for (const c of appStore.collections) {
      const r = c.requests.find((r) => r.id === requestId);
      if (r) return { collectionId: c.id, request: r };
    }
    return null;
  }

  function replayEntry(entry: RequestHistoryEntry) {
    editorStore.saveIfDirty();

    // If from a saved request that still exists, navigate to it
    if (entry.requestId) {
      const found = findSavedRequest(entry.requestId);
      if (found) {
        appStore.selectRequest(found.collectionId, found.request.id);
        editorStore.loadFrom(found.request);
        return;
      }
    }

    // Otherwise replay from snapshot (detached — use temp ID so editor renders)
    if (entry.snapshot) {
      editorStore.loadFrom({ ...entry.snapshot, id: crypto.randomUUID(), name: entry.requestName || "History Replay" });
      editorStore.isDirty = false;
    } else {
      // Fallback: only method + URL available (old history entries)
      editorStore.requestId = crypto.randomUUID();
      editorStore.name = entry.requestName || "History Replay";
      editorStore.method = entry.method;
      editorStore.url = entry.url;
      editorStore.queryParams = [createEmptyPair()];
      editorStore.headers = [createEmptyPair()];
      editorStore.bodyType = "none";
      editorStore.jsonBody = "";
      editorStore.rawBody = "";
      editorStore.formPairs = [createEmptyPair()];
      editorStore.response = null;
      editorStore.errorMessage = null;
      editorStore.isDirty = false;
    }
  }

  function formatTime(timestamp: string): string {
    const d = new Date(timestamp);
    const now = new Date();
    const diffMs = now.getTime() - d.getTime();
    const diffMin = Math.floor(diffMs / 60000);
    if (diffMin < 1) return "just now";
    if (diffMin < 60) return `${diffMin}m ago`;
    const diffHour = Math.floor(diffMin / 60);
    if (diffHour < 24) return `${diffHour}h ago`;
    return d.toLocaleDateString();
  }
</script>

{#if historyStore.history.length > 0}
  <div class="history-section">
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="header" onclick={() => (expanded = !expanded)}>
      <span class="chevron">{expanded ? "▾" : "▸"}</span>
      <span class="title">History</span>
      <span class="count">{historyStore.history.length}</span>
    </div>
    {#if expanded}
      <div class="entries">
        {#each recentHistory as entry (entry.id)}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="entry"
            class:linked={entry.requestId && findSavedRequest(entry.requestId)}
            onclick={() => replayEntry(entry)}
            title={entry.url}
          >
            <HttpMethodBadge method={entry.method} />
            <div class="entry-info">
              <span class="entry-label">
                {entry.requestName || entry.url}
              </span>
              <span class="entry-meta">
                {formatTime(entry.timestamp)}
                {#if entry.elapsedTime}· {Math.round(entry.elapsedTime)}ms{/if}
              </span>
            </div>
            {#if entry.statusCode}
              <span
                class="status"
                class:success={entry.statusCode >= 200 && entry.statusCode < 300}
                class:error={entry.statusCode >= 400}
              >
                {entry.statusCode}
              </span>
            {/if}
          </div>
        {/each}
        <button class="clear-btn" onclick={() => historyStore.clear()}>
          Clear History
        </button>
      </div>
    {/if}
  </div>
{/if}

<style>
  .history-section {
    border-top: 1px solid var(--border-color);
    margin-top: var(--sp-sm);
    padding-top: var(--sp-xs);
  }
  .header {
    display: flex;
    align-items: center;
    gap: var(--sp-xs);
    padding: var(--sp-xs) var(--sp-sm);
    cursor: pointer;
    font-size: var(--fs-small);
    font-weight: 600;
    color: var(--text-secondary);
  }
  .header:hover {
    background: var(--bg-hover);
    border-radius: var(--radius-sm);
  }
  .chevron {
    font-size: var(--fs-small);
    width: 14px;
    text-align: center;
  }
  .title {
    flex: 1;
  }
  .count {
    font-size: var(--fs-caption);
    color: var(--text-tertiary);
    font-weight: 400;
  }
  .entries {
    padding: 0 var(--sp-sm);
  }
  .entry {
    display: flex;
    align-items: center;
    gap: var(--sp-xs);
    padding: var(--sp-xs) var(--sp-sm);
    font-size: var(--fs-footnote);
    border-radius: var(--radius-sm);
    cursor: pointer;
  }
  .entry:hover {
    background: var(--bg-hover);
  }
  .entry.linked {
    border-left: 2px solid var(--color-info);
  }
  .entry-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 1px;
  }
  .entry-label {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--text-primary);
  }
  .entry-meta {
    font-size: var(--fs-caption);
    color: var(--text-tertiary);
  }
  .status {
    font-family: var(--font-mono);
    font-size: var(--fs-caption);
    flex-shrink: 0;
  }
  .status.success {
    color: var(--color-success);
  }
  .status.error {
    color: var(--color-error);
  }
  .clear-btn {
    font-size: var(--fs-caption);
    color: var(--text-tertiary);
    margin-top: var(--sp-xs);
    padding: 2px var(--sp-sm);
  }
</style>
