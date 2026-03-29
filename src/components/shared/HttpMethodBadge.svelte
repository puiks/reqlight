<script lang="ts">
  import { METHOD_COLORS, type HttpMethod } from "../../lib/types";

  let { method }: { method: HttpMethod } = $props();
</script>

<span class="badge" style="color: {METHOD_COLORS[method]}">
  {method}
</span>

<style>
  .badge {
    font-size: var(--fs-footnote);
    font-weight: 600;
    font-family: var(--font-mono);
    display: inline-flex;
    align-items: center;
    flex-shrink: 0;
    min-width: 32px;
    line-height: 1;
  }
</style>
