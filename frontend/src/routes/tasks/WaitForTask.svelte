<script lang="typescript">
    import SecondaryButton from "../../components/buttons/Secondary.svelte";
    import ActionRow from "../../components/buttons/ActionRow.svelte";

    export let leaveGame: () => Promise<void>;
</script>

<div class="grid md:grid-cols-3 grid-cols-1 gap-8 mb-8">
    <h4 class="font-bold mb-4">Wait for next Task</h4>
</div>
<ActionRow>
    <div />
    <SecondaryButton on:click={leaveGame}>Leave Game</SecondaryButton>
</ActionRow>
