<script lang="ts">
  import Modal from "../shared/Modal.svelte";
  import { appStore } from "../../lib/stores/app.svelte";

  let { onclose }: { onclose: () => void } = $props();
</script>

<Modal title="Settings" {onclose}>
  <div class="settings">
    <section class="section">
      <h3 class="section-title">Proxy</h3>

      <label class="toggle-row">
        <input
          type="checkbox"
          bind:checked={appStore.proxyConfig.enabled}
          onchange={() => appStore.scheduleSave()}
        />
        <span>Enable Proxy</span>
      </label>

      <div class="field" class:disabled={!appStore.proxyConfig.enabled}>
        <label class="label">
          Proxy URL
          <input
            type="text"
            class="input"
            placeholder="http://proxy:8080"
            bind:value={appStore.proxyConfig.proxyUrl}
            oninput={() => appStore.scheduleSave()}
            disabled={!appStore.proxyConfig.enabled}
          />
        </label>
      </div>

      <div class="field" class:disabled={!appStore.proxyConfig.enabled}>
        <label class="label">
          No Proxy
          <input
            type="text"
            class="input"
            placeholder="localhost, 127.0.0.1, .internal"
            bind:value={appStore.proxyConfig.noProxy}
            oninput={() => appStore.scheduleSave()}
            disabled={!appStore.proxyConfig.enabled}
          />
        </label>
        <span class="hint">Comma-separated hostnames to bypass proxy</span>
      </div>
    </section>
  </div>
</Modal>

<style>
  .settings {
    padding: var(--sp-sm) 0;
    min-width: 400px;
  }
  .section {
    display: flex;
    flex-direction: column;
    gap: var(--sp-md);
  }
  .section-title {
    font-size: var(--fs-callout);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }
  .toggle-row {
    display: flex;
    align-items: center;
    gap: var(--sp-sm);
    font-size: var(--fs-small);
    cursor: pointer;
  }
  .toggle-row input[type="checkbox"] {
    width: 16px;
    height: 16px;
    cursor: pointer;
  }
  .field {
    display: flex;
    flex-direction: column;
    gap: var(--sp-xs);
  }
  .field.disabled {
    opacity: 0.5;
  }
  .label {
    font-size: var(--fs-small);
    font-weight: 500;
    color: var(--text-secondary);
  }
  .input {
    font-size: var(--fs-small);
    padding: var(--sp-sm);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    background: var(--bg-primary);
    color: var(--text-primary);
  }
  .input:focus {
    border-color: var(--color-info);
    outline: none;
  }
  .hint {
    font-size: var(--fs-caption);
    color: var(--text-tertiary);
  }
</style>
