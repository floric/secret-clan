<script lang="typescript">
    import type { GameDetails } from "../../types/Game";
    import PrimaryButton from "../../components/buttons/Primary.svelte";
    import SecondaryButton from "../../components/buttons/Secondary.svelte";
    import ActionRow from "../../components/buttons/ActionRow.svelte";
    import TextInput from "../../components/inputs/TextInput.svelte";
    import Label from "../../components/inputs/Label.svelte";
    import DialogHeader from "../../components/headers/DialogHeader.svelte";
    import { getClaims } from "../../utils/auth";
    import { sendRequest } from "../../utils/requests";

    export let details: GameDetails;
    export let leaveGame: () => Promise<void>;
    export let refreshGame: () => Promise<void>;
    const claims = getClaims();
    const currentName = details.players[claims.sub].name;

    const startGame = async () => {
        await sendRequest(`/api/games/${details.game.token}/start`, "POST");
        await refreshGame();
    };

    const onChangeName = async (ev: any) => {
        const name = ev?.target?.value;
        if (!name) {
            return;
        }

        await sendRequest(`/api/tasks/settings`, "POST", { name });
        await refreshGame();
    };
</script>

<DialogHeader>Lobby</DialogHeader>
<div class="mb-8 flex items-center">
    <div>Token</div>
    <div
        class="font-extrabold mx-4 px-3 py-2 rounded-md bg-gray-200 border-black">
        {details.game.token.toUpperCase()}
    </div>
</div>
<div class="grid md:grid-cols-3 grid-cols-1 gap-8 mb-8">
    <div class="md:col-span-2">
        <h4 class="font-bold mb-4">Settings</h4>
        <div class="max-w-xs">
            <Label target="name">Your Name</Label>
            <TextInput
                id="name"
                placeholder="Name"
                value={currentName}
                on:change={onChangeName} />
        </div>
    </div>
    <div>
        <h4 class="font-bold mb-4">Players</h4>
        <ul>
            {#each Object.values(details.players) as p}
                <li>
                    {p.name}
                    {#if p.id === details.game.adminId}
                        <span class="font-bold">(Admin)</span>
                    {/if}
                </li>
            {/each}
        </ul>
    </div>
</div>
<ActionRow>
    {#if details.game.adminId === claims.sub}
        <PrimaryButton onClick={startGame}>Start</PrimaryButton>
    {:else}
        <p>Wait for the game to start.</p>
    {/if}
    <SecondaryButton onClick={leaveGame}>Leave Game</SecondaryButton>
</ActionRow>
