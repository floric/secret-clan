<script lang="typescript">
    export let id: string;
    export let value: string;
    export let placeholder: string;
</script>

<input
    {id}
    {placeholder}
    on:change
    bind:value
    class="text-sm sm:text-base placeholder-gray-500 rounded-lg border border-gray-400 w-full p-2 focus:outline-none focus:border-blue-400" />
