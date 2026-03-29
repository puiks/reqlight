<script lang="ts">
  let {
    label,
    value = $bindable(),
    placeholder = "",
    hint = "",
    disabled = false,
    oninput,
  }: {
    label: string;
    value: string;
    placeholder?: string;
    hint?: string;
    disabled?: boolean;
    oninput?: () => void;
  } = $props();
</script>

<!-- svelte-ignore a11y_label_has_associated_control -->
<label class="labeled-field" class:disabled>
  <span class="field-label">{label}</span>
  <span class="field-body">
    <input
      type="text"
      class="field-input"
      {placeholder}
      bind:value
      {disabled}
      oninput={oninput}
    />
    {#if hint}
      <span class="field-hint">{hint}</span>
    {/if}
  </span>
</label>

<style>
  .labeled-field {
    display: flex;
    align-items: flex-start;
    gap: var(--sp-md);
    cursor: default;
  }
  .labeled-field.disabled {
    opacity: 0.5;
  }
  .field-label {
    font-size: var(--fs-callout);
    font-weight: 500;
    color: var(--text-secondary);
    white-space: nowrap;
    min-width: 80px;
    flex-shrink: 0;
    padding-top: var(--sp-xs);
  }
  .field-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }
  .field-input {
    font-size: var(--fs-callout);
    padding: var(--sp-xs) var(--sp-sm);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    background: var(--bg-primary);
    color: var(--text-primary);
    width: 100%;
  }
  .field-input:focus {
    border-color: var(--color-info);
    outline: none;
  }
  .field-input::placeholder {
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .field-hint {
    font-size: var(--fs-small);
    color: var(--text-tertiary);
  }
</style>
