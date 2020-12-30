<script lang="typescript">
    import type { Role } from "../../types/Role";
    import SecondaryButton from "../../components/buttons/Secondary.svelte";
    import PrimaryButton from "../../components/buttons/Primary.svelte";
    import ActionRow from "../../components/buttons/ActionRow.svelte";
    import { sendRequest } from "../../utils/requests";

    export let leaveGame: () => Promise<void>;
    export let refreshGame: () => Promise<void>;
    export let role: Role;

    const acknowledge = async () => {
        await sendRequest(`/api/tasks/disclose-role`, "POST", {
            acknowledge: true,
        });
        await refreshGame();
    };
</script>

<div class="grid md:grid-cols-3 grid-cols-1 gap-8 mb-8">
    <h4 class="font-bold mb-4">Your role</h4>
    <p>Name: {role.name} | Party: {role.party}</p>
    <PrimaryButton on:click={acknowledge}>Acknowledge</PrimaryButton>
</div>
<ActionRow>
    <div />
    <SecondaryButton on:click={leaveGame}>Leave Game</SecondaryButton>
</ActionRow>
