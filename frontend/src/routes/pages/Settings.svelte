<script lang="typescript">
    import type { GameDetails } from "../../types/Game";
    import PrimaryButton from "../../components/buttons/Primary.svelte";
    import SecondaryButton from "../../components/buttons/Secondary.svelte";
    import ActionRow from "../../components/buttons/ActionRow.svelte";
    import TextInput from "../../components/inputs/TextInput.svelte";
    import Label from "../../components/inputs/Label.svelte";
    import { getClaims, getToken } from "../../utils/auth";

    export let details: GameDetails;
    export let token: string;
    export let leaveGame: () => Promise<void>;
    export let refreshGame: () => Promise<void>;
    const claims = getClaims();

    const startGame = async () => {
        await fetch(`/api/games/${token}/start`, {
            headers: {
                Authorization: `Bearer ${getToken()}`,
            },
            method: "POST",
        });
        await refreshGame();
    };

    const onChangeName = async (ev: any) => {
        const name = ev?.target?.value;
        if (!name) {
            return;
        }

        await fetch(`/api/players/${claims.sub}`, {
            headers: {
                Authorization: `Bearer ${getToken()}`,
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ name }),
            method: "POST",
        });
        await refreshGame();
    };
</script>

<div class="mb-8 flex items-center">
    <div>Token</div>
    <div
        class="font-extrabold mx-4 px-3 py-2 rounded-md bg-gray-200 border-black">
        {token.toUpperCase()}
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
                value={[...details.participants.players, details.participants.admin].find((x) => x.id === claims.sub).name}
                on:change={onChangeName} />
        </div>
    </div>
    <div>
        <h4 class="font-bold mb-4">Players</h4>
        <ul>
            <li>
                {details.participants.admin.name}
                <span class="font-bold">(Admin)</span>
            </li>
            {#each details.participants.players as p}
                <li>{p.name}</li>
            {/each}
        </ul>
    </div>
</div>
<ActionRow>
    {#if details.game.admin_id === claims.sub}
        <PrimaryButton on:click={startGame}>Start</PrimaryButton>
    {:else}
        <p>Wait for the game to start.</p>
    {/if}
    <SecondaryButton on:click={leaveGame}>Leave Game</SecondaryButton>
</ActionRow>
