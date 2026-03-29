<script lang="ts">
  import RequestRow from "./RequestRow.svelte";
  import type { RequestCollection } from "../../lib/types";
  import { appStore } from "../../lib/stores/app.svelte";
  import { editorStore } from "../../lib/stores/editor.svelte";

  let {
    collection,
    oncontextmenu,
    oncontextrequest,
    ondelete,
    ondeleterequest,
    onrun,
    onaddrequest,
  }: {
    collection: RequestCollection;
    oncontextmenu?: (e: MouseEvent) => void;
    oncontextrequest?: (e: MouseEvent, requestId: string) => void;
    ondelete?: () => void;
    ondeleterequest?: (requestId: string, requestName: string) => void;
    onrun?: () => void;
    onaddrequest?: () => void;
  } = $props();

  let expanded = $state(true);
  let dragOverIndex = $state<number | null>(null);
  let dragFromIndex = $state<number | null>(null);

  function toggleExpand() {
    expanded = !expanded;
  }

  function selectRequest(requestId: string) {
    appStore.selectRequest(collection.id, requestId);
    const request = collection.requests.find((r) => r.id === requestId);
    if (request) editorStore.loadFrom(request);
  }

  function handleDragStart(index: number, e: DragEvent) {
    dragFromIndex = index;
    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = "move";
      e.dataTransfer.setData("text/plain", String(index));
    }
  }

  function handleDragOver(index: number, e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) e.dataTransfer.dropEffect = "move";
    dragOverIndex = index;
  }

  function handleDrop(index: number, e: DragEvent) {
    e.preventDefault();
    if (dragFromIndex !== null && dragFromIndex !== index) {
      appStore.reorderRequest(collection.id, dragFromIndex, index);
    }
    dragFromIndex = null;
    dragOverIndex = null;
  }

  function handleDragEnd() {
    dragFromIndex = null;
    dragOverIndex = null;
  }
</script>

<div class="collection">
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="header" onclick={toggleExpand} oncontextmenu={oncontextmenu}>
    <span class="chevron" class:expanded>{expanded ? "▾" : "▸"}</span>
    <span class="name">{collection.name}</span>
    <span class="count">{collection.requests.length}</span>
    <span class="header-actions">
      {#if onaddrequest}
        <button
          class="header-action-btn add-req-btn"
          title="Add Request"
          aria-label="Add Request"
          onclick={(e) => { e.stopPropagation(); onaddrequest?.(); }}
        >+</button>
      {/if}
      {#if onrun}
        <button
          class="header-action-btn run-btn"
          title="Run Collection"
          aria-label="Run Collection"
          onclick={(e) => { e.stopPropagation(); onrun?.(); }}
        >▶</button>
      {/if}
      {#if ondelete}
        <button
          class="header-action-btn delete-btn"
          title="Delete Collection"
          aria-label="Delete Collection"
          onclick={(e) => { e.stopPropagation(); ondelete?.(); }}
        >✕</button>
      {/if}
    </span>
  </div>
  {#if expanded}
    <div class="requests">
      {#each collection.requests as request, index (request.id)}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="drag-wrapper"
          class:drag-over={dragOverIndex === index && dragFromIndex !== index}
          class:dragging={dragFromIndex === index}
          draggable="true"
          ondragstart={(e) => handleDragStart(index, e)}
          ondragover={(e) => handleDragOver(index, e)}
          ondrop={(e) => handleDrop(index, e)}
          ondragend={handleDragEnd}
          ondragleave={() => { if (dragOverIndex === index) dragOverIndex = null; }}
        >
          <RequestRow
            {request}
            isSelected={appStore.selectedRequestId === request.id}
            onclick={() => selectRequest(request.id)}
            oncontextmenu={(e) => { e.preventDefault(); oncontextrequest?.(e, request.id); }}
            ondelete={() => ondeleterequest?.(request.id, request.name)}
          />
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .collection {
    margin-bottom: 2px;
  }
  .header {
    display: flex;
    align-items: center;
    gap: var(--sp-xs);
    padding: var(--sp-xs) var(--sp-sm);
    cursor: pointer;
    border-radius: var(--radius-sm);
    font-size: var(--fs-callout);
    font-weight: 600;
    position: relative;
  }
  .header:hover {
    background: var(--bg-hover);
  }
  .chevron {
    font-size: var(--fs-small);
    color: var(--text-secondary);
    width: 14px;
    text-align: center;
  }
  .name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .count {
    font-size: var(--fs-caption);
    color: var(--text-tertiary);
    font-weight: 400;
  }
  .header-actions {
    display: none;
    position: absolute;
    right: var(--sp-xs);
    top: 50%;
    transform: translateY(-50%);
    align-items: center;
    background: var(--bg-hover);
    border-radius: var(--radius-sm);
  }
  .header:hover .count {
    display: none;
  }
  .header:hover .header-actions {
    display: inline-flex;
  }
  .header-action-btn {
    font-size: var(--fs-small);
    color: var(--text-tertiary);
    padding: 2px;
    min-width: 22px;
    min-height: 22px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    line-height: 1;
    border-radius: var(--radius-sm);
  }
  .add-req-btn {
    font-size: var(--fs-callout);
    font-weight: 600;
  }
  .add-req-btn:hover {
    color: var(--color-info);
    background: var(--bg-tertiary);
  }
  .run-btn:hover {
    color: var(--color-success);
    background: var(--bg-tertiary);
  }
  .delete-btn:hover {
    color: var(--color-error);
    background: var(--bg-tertiary);
  }
  .requests {
    padding-left: var(--sp-xs);
  }
  .drag-wrapper {
    border-top: 2px solid transparent;
    transition: border-color 0.1s;
  }
  .drag-wrapper.drag-over {
    border-top-color: var(--color-info);
  }
  .drag-wrapper.dragging {
    opacity: 0.4;
  }
</style>
