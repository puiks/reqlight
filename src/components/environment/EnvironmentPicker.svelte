<script lang="ts">
  import { environmentStore } from "../../lib/stores/environment.svelte";

  let { onmanage }: { onmanage: () => void } = $props();
</script>

<div class="env-picker">
  <select
    class="env-select"
    value={environmentStore.activeEnvironmentId ?? ""}
    onchange={(e) => {
      const val = e.currentTarget.value;
      environmentStore.setActiveEnvironment(val || null);
    }}
  >
    <option value="">No Environment</option>
    {#each environmentStore.environments as env}
      <option value={env.id}>{env.name}</option>
    {/each}
  </select>
  <button class="manage-btn" onclick={onmanage} title="Manage Environments (⌘E)">
    Manage
  </button>
</div>

<style>
  .env-picker {
    display: flex;
    align-items: center;
    gap: var(--sp-xs);
  }
  .env-select {
    font-size: var(--fs-callout);
    background: var(--bg-input);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    padding: 2px 6px;
    color: var(--text-primary);
    cursor: pointer;
  }
  .manage-btn {
    font-size: var(--fs-callout);
    color: var(--text-secondary);
    padding: 2px 6px;
    border-radius: var(--radius-sm);
    white-space: nowrap;
    min-height: 28px;
  }
  .manage-btn:hover {
    color: var(--text-primary);
    background: var(--bg-tertiary);
  }
</style>
