<script lang="typescript">
    export let id = "";
    export let value = "";
    export let placeholder = "";
</script>

<input
    {id}
    {placeholder}
    on:change
    bind:value
    class="text-sm sm:text-base placeholder-gray-500 rounded-lg border border-gray-400 w-full py-2 px-3 focus:outline-none focus:border-blue-400" />
