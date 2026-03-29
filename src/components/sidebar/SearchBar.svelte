<script lang="ts">
  import { appStore } from "../../lib/stores/app.svelte";
  import SearchInput from "../shared/SearchInput.svelte";
</script>

<SearchInput
  bind:value={appStore.searchQuery}
  placeholder="Search collections & requests..."
/>
