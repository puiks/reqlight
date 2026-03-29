<script lang="ts">
  import { appStore } from "../../lib/stores/app.svelte";
  import EnvironmentPicker from "../environment/EnvironmentPicker.svelte";

  let {
    onopenenvs,
    onopensettings,
  }: {
    onopenenvs: () => void;
    onopensettings: () => void;
  } = $props();

  function toggleSidebar() {
    appStore.sidebarVisible = !appStore.sidebarVisible;
  }
</script>

<div class="toolbar" data-tauri-drag-region>
  <div class="left">
    <button class="icon-btn" onclick={toggleSidebar} title="Toggle Sidebar">
      ☰
    </button>
    <button class="icon-btn" onclick={onopensettings} title="Settings">
      ⚙
    </button>
  </div>

  <div class="center" data-tauri-drag-region></div>

  <div class="right">
    <EnvironmentPicker onmanage={onopenenvs} />
  </div>
</div>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 38px;
    padding: 0 var(--sp-md);
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    flex-shrink: 0;
  }
  .left,
  .right {
    display: flex;
    align-items: center;
  }
  .center {
    flex: 1;
  }
  .icon-btn {
    font-size: var(--fs-title3);
    padding: 2px;
    color: var(--text-secondary);
    min-width: 26px;
    min-height: 26px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }
  .icon-btn:hover {
    color: var(--text-primary);
    background: var(--bg-tertiary);
    border-radius: var(--radius-sm);
  }
</style>
