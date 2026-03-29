<script lang="ts">
  import { editorStore } from "../../lib/stores/editor.svelte";
  import { exportCurl } from "../../lib/commands";
  import { environmentStore } from "../../lib/stores/environment.svelte";
  import { toastStore } from "../../lib/stores/toast.svelte";
  import { HTTP_METHODS, METHOD_COLORS, type HttpMethod } from "../../lib/types";

  let { onimportcurl }: { onimportcurl: () => void } = $props();

  let showCurlMenu = $state(false);

  function handleSend() {
    editorStore.send();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) {
      handleSend();
    }
  }

  function toggleCurlMenu() {
    showCurlMenu = !showCurlMenu;
  }

  function handleImportCurl() {
    showCurlMenu = false;
    onimportcurl();
  }

  async function handleExportCurl() {
    showCurlMenu = false;
    const request = editorStore.toSavedRequest();
    if (!request) {
      toastStore.show("No request to export");
      return;
    }
    try {
      const activeEnv = environmentStore.environments.find(
        (e) => e.id === environmentStore.activeEnvironmentId,
      );
      const curl = await exportCurl(request, activeEnv);
      await navigator.clipboard.writeText(curl);
      toastStore.show("cURL copied to clipboard");
    } catch (e) {
      toastStore.show("Failed to export cURL");
    }
  }
</script>

<svelte:window onclick={() => (showCurlMenu = false)} />

<div class="url-bar">
  <div class="curl-wrapper">
    <button
      class="import-curl-btn"
      onclick={(e) => { e.stopPropagation(); toggleCurlMenu(); }}
      title="cURL Import / Export"
    >
      cURL
    </button>
    {#if showCurlMenu}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="curl-menu" onclick={(e) => e.stopPropagation()}>
        <button class="curl-menu-item" onclick={handleImportCurl}>
          Import from cURL
        </button>
        <button class="curl-menu-item" onclick={handleExportCurl}>
          Copy as cURL
        </button>
      </div>
    {/if}
  </div>

  <select
    class="method-select"
    bind:value={editorStore.method}
    onchange={() => editorStore.markDirty()}
    style="color: {METHOD_COLORS[editorStore.method]}"
  >
    {#each HTTP_METHODS as m}
      <option value={m} style="color: {METHOD_COLORS[m]}">{m}</option>
    {/each}
  </select>

  <input
    type="text"
    class="url-input"
    placeholder="Enter URL (e.g. https://api.example.com)"
    bind:value={editorStore.url}
    oninput={() => editorStore.markDirty()}
    onkeydown={handleKeydown}
    class:invalid={!editorStore.isUrlValid}
  />

  {#if editorStore.isLoading}
    <button class="cancel-btn" onclick={() => editorStore.cancel()}>Cancel</button>
  {:else}
    <button
      class="send-btn"
      onclick={handleSend}
      disabled={!editorStore.canSend}
      title="Send (⌘↩)"
    >
      Send
    </button>
  {/if}
</div>

<style>
  .url-bar {
    display: flex;
    gap: var(--sp-sm);
    padding: var(--sp-md);
    border-bottom: 1px solid var(--border-color);
    align-items: center;
  }
  .curl-wrapper {
    position: relative;
    flex-shrink: 0;
  }
  .import-curl-btn {
    font-size: var(--fs-callout);
    font-family: var(--font-mono);
    padding: 2px 6px;
    color: var(--text-tertiary);
    border-radius: var(--radius-sm);
    white-space: nowrap;
    border: 1px dashed var(--border-color);
    min-height: 28px;
  }
  .import-curl-btn:hover {
    color: var(--text-primary);
    border-color: var(--text-secondary);
  }
  .curl-menu {
    position: absolute;
    top: 100%;
    left: 0;
    margin-top: var(--sp-xs);
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-elevated);
    min-width: 160px;
    z-index: 50;
    overflow: hidden;
  }
  .curl-menu-item {
    display: block;
    width: 100%;
    text-align: left;
    padding: var(--sp-sm) var(--sp-md);
    font-size: var(--fs-callout);
    color: var(--text-primary);
    white-space: nowrap;
  }
  .curl-menu-item:hover {
    background: var(--bg-hover);
  }
  .method-select {
    font-family: var(--font-mono);
    font-weight: 600;
    font-size: var(--fs-body);
    background: var(--bg-input);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    padding: var(--sp-xs) var(--sp-sm);
    width: 90px;
    cursor: pointer;
  }
  .url-input {
    flex: 1;
    font-family: var(--font-mono);
    font-size: var(--fs-body);
  }
  .url-input.invalid {
    border-color: var(--color-error);
  }
  .send-btn {
    background: var(--color-info);
    color: white;
    font-weight: 600;
    padding: var(--sp-xs) var(--sp-lg);
    border-radius: var(--radius-sm);
  }
  .send-btn:hover:not(:disabled) {
    opacity: 0.9;
    background: var(--color-info);
  }
  .send-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .cancel-btn {
    background: var(--bg-tertiary);
    font-weight: 600;
    padding: var(--sp-xs) var(--sp-lg);
    border-radius: var(--radius-sm);
  }
</style>
