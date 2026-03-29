<script lang="ts">
  interface Props {
    error: unknown;
    onreset: () => void;
  }

  const { error, onreset }: Props = $props();

  const message = $derived(
    error instanceof Error ? error.message : String(error),
  );
</script>

<div class="error-fallback">
  <div class="error-icon">!</div>
  <h2>Something went wrong</h2>
  <p class="error-message">{message}</p>
  <div class="actions">
    <button class="btn-retry" onclick={onreset}>Retry</button>
    <button class="btn-reload" onclick={() => window.location.reload()}>
      Reload App
    </button>
  </div>
</div>

<style>
  .error-fallback {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: var(--sp-xl);
    text-align: center;
    color: var(--text-secondary);
  }

  .error-icon {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    background: var(--color-error);
    color: white;
    font-size: var(--fs-title2);
    font-weight: 700;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: var(--sp-lg);
  }

  h2 {
    margin: 0 0 var(--sp-sm);
    color: var(--text-primary);
    font-size: var(--fs-subhead);
  }

  .error-message {
    margin: 0 0 var(--sp-lg);
    font-family: var(--font-mono);
    font-size: var(--fs-callout);
    max-width: 500px;
    word-break: break-word;
  }

  .actions {
    display: flex;
    gap: var(--sp-sm);
  }

  .actions button {
    padding: var(--sp-sm) var(--sp-lg);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
    cursor: pointer;
    font-size: var(--fs-callout);
  }

  .btn-retry {
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .btn-retry:hover {
    background: var(--bg-tertiary);
  }

  .btn-reload {
    background: var(--color-info);
    color: white;
    border-color: var(--color-info);
  }

  .btn-reload:hover {
    opacity: 0.9;
  }
</style>
