<script lang="ts">
    import { get, increment } from "./Counters.svelte";
</script>

<button on:click={increment}>
    count is {get()}
</button>
