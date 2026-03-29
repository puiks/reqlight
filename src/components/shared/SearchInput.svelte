<script lang="ts">
  let {
    value = $bindable(""),
    placeholder = "Search...",
    oninput,
  }: {
    value?: string;
    placeholder?: string;
    oninput?: (e: Event) => void;
  } = $props();
</script>

<div class="search-input">
  <input
    type="text"
    {placeholder}
    bind:value
    oninput={oninput}
  />
</div>

<style>
  .search-input {
    flex: 1;
    min-width: 0;
  }
  input {
    width: 100%;
    font-size: var(--fs-callout);
    padding: var(--sp-xs) var(--sp-sm);
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
