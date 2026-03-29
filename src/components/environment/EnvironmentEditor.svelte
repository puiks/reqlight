<script lang="ts">
  import { environmentStore } from "../../lib/stores/environment.svelte";
  import KeyValueEditor from "../editor/KeyValueEditor.svelte";
  import Modal from "../shared/Modal.svelte";

  let { onclose }: { onclose: () => void } = $props();

  let selectedEnvId = $state<string | null>(
    environmentStore.environments[0]?.id ?? null,
  );

  const selectedEnv = $derived(
    environmentStore.environments.find((e) => e.id === selectedEnvId),
  );

  function addEnvironment() {
    const env = environmentStore.addEnvironment();
    selectedEnvId = env.id;
  }

  function deleteEnvironment() {
    if (!selectedEnvId) return;
    environmentStore.deleteEnvironment(selectedEnvId);
    selectedEnvId = environmentStore.environments[0]?.id ?? null;
  }

  function handleNameChange(e: Event) {
    if (!selectedEnv) return;
    const name = (e.target as HTMLInputElement).value;
    environmentStore.updateEnvironment({ ...selectedEnv, name });
  }

  function handleVarsChange() {
    if (!selectedEnv) return;
    environmentStore.updateEnvironment(selectedEnv);
  }
</script>

<Modal title="Environments" {onclose} noPadding>
  <div class="env-editor">
    <div class="env-list">
      <div class="list-items">
        {#each environmentStore.environments as env (env.id)}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="env-item"
            class:selected={selectedEnvId === env.id}
            onclick={() => (selectedEnvId = env.id)}
          >
            {env.name}
          </div>
        {/each}
        {#if environmentStore.environments.length === 0}
          <div class="no-envs">No environments yet.</div>
        {/if}
      </div>
      <div class="list-footer">
        <button class="add-btn" onclick={addEnvironment}>+ New Environment</button>
      </div>
    </div>

    <div class="env-detail">
      {#if selectedEnv}
        <div class="detail-header">
          <input
            type="text"
            class="env-name-input"
            value={selectedEnv.name}
            oninput={handleNameChange}
            placeholder="Environment name"
          />
          <button class="delete-btn" onclick={deleteEnvironment} title="Delete Environment">
            ✕
          </button>
        </div>
        <div class="vars-section">
          <KeyValueEditor
            bind:pairs={selectedEnv.variables}
            showSecret={true}
            onchange={handleVarsChange}
          />
        </div>
      {:else}
        <div class="no-selection">Select an environment to edit.</div>
      {/if}
    </div>
  </div>
</Modal>

<style>
  .env-editor {
    display: flex;
    min-height: 360px;
    min-width: 560px;
  }
  .env-list {
    width: 180px;
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }
  .list-items {
    flex: 1;
    overflow-y: auto;
    padding: var(--sp-xs);
  }
  .list-footer {
    padding: var(--sp-sm);
    border-top: 1px solid var(--border-light);
  }
  .add-btn {
    font-size: var(--fs-callout);
    color: var(--color-info);
    font-weight: 600;
    width: 100%;
    text-align: left;
    padding: var(--sp-xs) var(--sp-sm);
    border-radius: var(--radius-sm);
  }
  .add-btn:hover {
    background: var(--bg-hover);
  }
  .env-item {
    padding: var(--sp-sm) var(--sp-md);
    font-size: var(--fs-callout);
    cursor: pointer;
    border-radius: var(--radius-sm);
  }
  .env-item:hover {
    background: var(--bg-hover);
  }
  .env-item.selected {
    background: var(--bg-selected);
    color: var(--color-info);
    font-weight: 600;
  }
  .no-envs {
    padding: var(--sp-md);
    font-size: var(--fs-callout);
    color: var(--text-tertiary);
    text-align: center;
  }
  .env-detail {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .detail-header {
    display: flex;
    align-items: center;
    gap: var(--sp-sm);
    padding: var(--sp-sm) var(--sp-md);
    border-bottom: 1px solid var(--border-light);
  }
  .env-name-input {
    flex: 1;
    font-size: var(--fs-callout);
    font-weight: 600;
    padding: var(--sp-xs) var(--sp-sm);
  }
  .delete-btn {
    font-size: var(--fs-title3);
    color: var(--text-tertiary);
    padding: 2px 6px;
    min-width: 28px;
    min-height: 28px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-sm);
    flex-shrink: 0;
  }
  .delete-btn:hover {
    color: var(--color-error);
    background: var(--bg-hover);
  }
  .vars-section {
    flex: 1;
    overflow: auto;
    padding: var(--sp-sm);
  }
  .no-selection {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-tertiary);
    font-size: var(--fs-callout);
  }
</style>
