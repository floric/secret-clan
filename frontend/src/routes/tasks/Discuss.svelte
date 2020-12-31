<script lang="typescript">
    import SecondaryButton from "../../components/buttons/Secondary.svelte";
    import ActionRow from "../../components/buttons/ActionRow.svelte";
    import DialogHeader from "../../components/headers/DialogHeader.svelte";

    export let leaveGame: () => Promise<void>;
</script>

<DialogHeader>Discuss</DialogHeader>
<div class="mb-4">Let's talk about the past events...</div>
<ActionRow>
    <div />
    <SecondaryButton onClick={leaveGame}>Leave Game</SecondaryButton>
</ActionRow>
