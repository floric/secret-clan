<script lang="typescript">
  export let isActive: boolean;
</script>

{#if !isActive}
  <span class="font-bold text-red-800">(Inactive)</span>
{/if}
