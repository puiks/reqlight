<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    spacing = "md",
    direction = "vertical",
    align,
    children,
  }: {
    spacing?: "xs" | "sm" | "md" | "lg" | "xl";
    direction?: "vertical" | "horizontal";
    align?: "start" | "center" | "end" | "stretch";
    children: Snippet;
  } = $props();

  const gapMap: Record<string, string> = {
    xs: "var(--sp-xs)",
    sm: "var(--sp-sm)",
    md: "var(--sp-md)",
    lg: "var(--sp-lg)",
    xl: "var(--sp-xl)",
  };

  const alignMap: Record<string, string> = {
    start: "flex-start",
    center: "center",
    end: "flex-end",
    stretch: "stretch",
  };
</script>

<div
  class="stack"
  style="gap: {gapMap[spacing]}; flex-direction: {direction === 'horizontal' ? 'row' : 'column'}; {align ? `align-items: ${alignMap[align]}` : ''}"
>
  {@render children()}
</div>

<style>
  .stack {
    display: flex;
  }
</style>
